import { useEffect, useState, useCallback } from "react";
import { Pause, Play, X, Download, AlertCircle } from "lucide-react";
import { cn } from "../lib/utils";
import type { DownloadInfo } from "../lib/types";
import {
  formatBytes,
  formatSpeed,
  formatEta,
  getDownloadStatus,
  pauseDownload,
  resumeDownload,
  cancelDownload,
} from "../lib/api";
import { Button } from "./ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";

interface DownloadProgressProps {
  gid: string;
  onComplete?: () => void;
  onRemove?: () => void;
}

export function DownloadProgressItem({
  gid,
  onComplete,
  onRemove,
}: DownloadProgressProps) {
  const [info, setInfo] = useState<DownloadInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  const fetchStatus = useCallback(async () => {
    try {
      const status = await getDownloadStatus(gid);
      setInfo(status);
      setError(null);

      if (status.status === "complete" && onComplete) {
        onComplete();
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to get status");
    }
  }, [gid, onComplete]);

  useEffect(() => {
    fetchStatus();
    const interval = setInterval(fetchStatus, 1000);
    return () => clearInterval(interval);
  }, [fetchStatus]);

  const handlePause = async () => {
    try {
      await pauseDownload(gid);
      fetchStatus();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to pause");
    }
  };

  const handleResume = async () => {
    try {
      await resumeDownload(gid);
      fetchStatus();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to resume");
    }
  };

  const handleCancel = async () => {
    try {
      await cancelDownload(gid);
      if (onRemove) onRemove();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to cancel");
    }
  };

  if (!info) {
    return (
      <div className="flex items-center gap-2 p-4 bg-muted rounded-lg">
        <div className="animate-spin h-4 w-4 border-2 border-primary border-t-transparent rounded-full" />
        <span className="text-sm text-muted-foreground">Loading...</span>
      </div>
    );
  }

  const progressPercent = Math.min(info.progress, 100);
  const isActive = info.status === "active";
  const isPaused = info.status === "paused";
  const isComplete = info.status === "complete";
  const isError = info.status === "error";

  return (
    <div className="p-4 bg-muted rounded-lg space-y-3">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1 min-w-0">
          <p className="font-medium truncate">{info.filename}</p>
          <div className="flex items-center gap-2 text-sm text-muted-foreground mt-1">
            {isActive && (
              <>
                <span>{formatBytes(info.downloaded)} / {formatBytes(info.total_size)}</span>
                <span>•</span>
                <span>{formatSpeed(info.speed)}</span>
                <span>•</span>
                <span>ETA: {formatEta(info.total_size, info.downloaded, info.speed)}</span>
              </>
            )}
            {isPaused && <span>Paused</span>}
            {isComplete && <span className="text-green-500">Complete</span>}
            {isError && (
              <span className="text-red-500 flex items-center gap-1">
                <AlertCircle className="h-3 w-3" />
                {info.error_message || "Error"}
              </span>
            )}
          </div>
        </div>

        <div className="flex items-center gap-2">
          {(isActive || isPaused) && (
            <>
              {isActive ? (
                <Button variant="ghost" size="icon" onClick={handlePause}>
                  <Pause className="h-4 w-4" />
                </Button>
              ) : (
                <Button variant="ghost" size="icon" onClick={handleResume}>
                  <Play className="h-4 w-4" />
                </Button>
              )}
              <Button variant="ghost" size="icon" onClick={handleCancel}>
                <X className="h-4 w-4" />
              </Button>
            </>
          )}
          {(isComplete || isError) && onRemove && (
            <Button variant="ghost" size="icon" onClick={onRemove}>
              <X className="h-4 w-4" />
            </Button>
          )}
        </div>
      </div>

      <div className="space-y-1">
        <div className="h-2 bg-background rounded-full overflow-hidden">
          <div
            className={cn(
              "h-full transition-all duration-300",
              isComplete ? "bg-green-500" : isError ? "bg-red-500" : "bg-primary"
            )}
            style={{ width: `${progressPercent}%` }}
          />
        </div>
        <div className="flex justify-between text-xs text-muted-foreground">
          <span>{progressPercent.toFixed(1)}%</span>
          <span>
            {info.status === "active" ? "Downloading" : info.status}
          </span>
        </div>
      </div>

      {error && (
        <p className="text-sm text-red-500">{error}</p>
      )}
    </div>
  );
}

interface DownloadListProps {
  downloads: string[];
  onRemove: (gid: string) => void;
}

export function DownloadList({ downloads, onRemove }: DownloadListProps) {
  if (downloads.length === 0) {
    return null;
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Download className="h-5 w-5" />
          Downloads ({downloads.length})
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        {downloads.map((gid) => (
          <DownloadProgressItem
            key={gid}
            gid={gid}
            onRemove={() => onRemove(gid)}
          />
        ))}
      </CardContent>
    </Card>
  );
}
