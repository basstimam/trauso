import { useState, useEffect, useCallback, useRef } from "react";
import {
  DownloadParams,
  TeraboxFileInfo,
} from "../lib/types";
import {
  getDownloadLink,
  addDownload,
  getDownloadStatus,
  startAria2,
  cancelDownload,
} from "../lib/api";

export type QueueStatus = "pending" | "fetching_link" | "downloading" | "completed" | "error" | "cancelled";

export interface QueueItem {
  id: string;
  file: TeraboxFileInfo;
  status: QueueStatus;
  gid?: string;
  progress: number;
  speed: number;
  downloaded: number;
  total: number;
  error?: string;
  params: DownloadParams;
  downloadDir?: string;
}

export function useDownloadQueue() {
  const [queue, setQueue] = useState<QueueItem[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);
  const [maxConcurrent, setMaxConcurrent] = useState(1);
  
  const queueRef = useRef(queue);
  queueRef.current = queue;

  const addToQueue = useCallback((
    files: TeraboxFileInfo[], 
    params: Omit<DownloadParams, "fs_id">,
    downloadDir?: string
  ) => {
    setQueue((prev) => {
      const newItems = files
        .filter((f) => !prev.some((p) => p.id === f.fs_id))
        .map((file) => ({
          id: file.fs_id,
          file,
          status: "pending" as QueueStatus,
          progress: 0,
          speed: 0,
          downloaded: 0,
          total: file.size || 0,
          params: { ...params, fs_id: file.fs_id },
          downloadDir
        }));
      return [...prev, ...newItems];
    });
  }, []);

  const removeFromQueue = useCallback(async (id: string) => {
    const item = queueRef.current.find(i => i.id === id);
    if (!item) return;

    if (item.status === 'downloading' && item.gid) {
      try {
        await cancelDownload(item.gid);
      } catch (e) {
        console.error("Failed to cancel download:", e);
      }
    }

    setQueue(prev => prev.filter(i => i.id !== id));
  }, []);

  const updateItem = (id: string, updates: Partial<QueueItem>) => {
    setQueue((prev) =>
      prev.map((item) => (item.id === id ? { ...item, ...updates } : item))
    );
  };

  const startItem = useCallback(async (id: string) => {
    const item = queueRef.current.find(i => i.id === id);
    if (!item || item.status !== 'pending') return;

    updateItem(id, { status: "fetching_link" });

    try {
      const linkResult = await getDownloadLink(item.params);

      if (!linkResult.ok || !linkResult.download_link) {
        throw new Error(linkResult.error_message || "Failed to get download link");
      }

      updateItem(id, { status: "downloading" });
      await startAria2();
      const gid = await addDownload(
        linkResult.download_link,
        item.downloadDir,
        item.file.name
      );

      updateItem(id, { gid });
    } catch (err) {
      updateItem(id, {
        status: "error",
        error: err instanceof Error ? err.message : "Unknown error",
      });
    }
  }, []);

  // Queue Processor
  useEffect(() => {
    if (!isProcessing) return;

    const currentQueue = queue; // Use state queue for dependency tracking
    const activeCount = currentQueue.filter(i => i.status === 'downloading' || i.status === 'fetching_link').length;
    
    if (activeCount < maxConcurrent) {
      const nextItem = currentQueue.find(i => i.status === 'pending');
      if (nextItem) {
        startItem(nextItem.id);
      }
    }
  }, [queue, isProcessing, maxConcurrent, startItem]);

  // Polling Loop for ALL active items
  useEffect(() => {
    const interval = setInterval(async () => {
      // Find all active items with GID
      const activeItems = queueRef.current.filter(i => i.status === 'downloading' && i.gid);
      
      for (const item of activeItems) {
        if (!item.gid) continue;
        
        try {
          const status = await getDownloadStatus(item.gid);
          
          updateItem(item.id, {
            progress: status.progress,
            speed: status.speed,
            downloaded: status.downloaded,
            total: status.total_size,
          });

          if (status.status === "complete") {
            updateItem(item.id, { status: "completed", progress: 100 });
          } else if (status.status === "error" || status.status === "removed") {
             // Removed status handling
             updateItem(item.id, { 
               status: status.status === "error" ? "error" : "cancelled", 
               error: status.error_message || (status.status === "removed" ? "Cancelled" : "Aria2 error") 
             });
          }
        } catch (e) {
          console.error(`Polling error for ${item.id}:`, e);
        }
      }
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return {
    queue,
    addToQueue,
    startQueue: () => setIsProcessing(true),
    pauseQueue: () => setIsProcessing(false),
    removeFromQueue,
    isProcessing,
    clearCompleted: () => setQueue(q => q.filter(i => i.status !== "completed")),
    clearAll: () => setQueue(q => q.filter(i => i.status === "downloading" || i.status === "fetching_link")),
    setMaxConcurrent,
    maxConcurrent
  };
}
