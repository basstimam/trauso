import { open } from "@tauri-apps/plugin-dialog";
import { useState, useCallback, useEffect } from "react";
import { check } from '@tauri-apps/plugin-updater';
import { relaunch, exit } from '@tauri-apps/plugin-process';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { confirm } from '@tauri-apps/plugin-dialog';
import { Download, Loader2, Link, FileDown, Trash2, Play, Pause, FolderOpen, Settings, X, ListX } from "lucide-react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { FileList, SelectAllButton } from "./file-list";
import { getTeraboxInfo, formatBytes, stopAria2 } from "../lib/api";
import { loadSettings, saveSettings } from "../lib/settings";
import { TeraboxInfo } from "../lib/types";
import { useDownloadQueue, QueueItem } from "../hooks/use-download-queue";
import { cn } from "../lib/utils";

export function DashboardLayout() {
  const [url, setUrl] = useState("");
  const [isFetching, setIsFetching] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [teraboxInfo, setTeraboxInfo] = useState<TeraboxInfo | null>(null);
  const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set());
  const [downloadDir, setDownloadDir] = useState("");
  const [downloadMode, setDownloadMode] = useState(2);
  const [showSettings, setShowSettings] = useState(false);

  const {
    queue,
    addToQueue,
    startQueue,
    pauseQueue,
    removeFromQueue,
    isProcessing,
    clearCompleted,
    clearAll,
    setMaxConcurrent,
    maxConcurrent
  } = useDownloadQueue();

  // Load Settings
  useEffect(() => {
    loadSettings().then(settings => {
      setDownloadDir(settings.downloadDir);
      setDownloadMode(settings.downloadMode);
      setMaxConcurrent(settings.maxConcurrent);
    });
  }, [setMaxConcurrent]);

  const handleSaveSettings = () => {
    saveSettings({
      downloadDir,
      downloadMode,
      maxConcurrent
    });
    setShowSettings(false);
  };

  // Auto-Update Check
  useEffect(() => {
    const initUpdate = async () => {
      try {
        const update = await check();
        if (update) {
          const yes = await confirm(
            `Update to version ${update.version} is available!\n\n${update.body || ''}`,
            { title: 'Update Available', kind: 'info' }
          );
          if (yes) {
            await update.downloadAndInstall();
            await relaunch();
          }
        }
      } catch (e) {
        console.error("Update check failed:", e);
      }
    };
    initUpdate();
  }, []);

  // Graceful Exit Handler
  useEffect(() => {
    const setupExitHandler = async () => {
      const appWindow = getCurrentWindow();
      const unlisten = await appWindow.onCloseRequested(async (event) => {
        event.preventDefault();
        const yes = await confirm(
          "Are you sure you want to quit?\nThis will stop all active downloads and kill the background process.",
          { title: 'Quit Trauso', kind: 'warning', okLabel: 'Quit', cancelLabel: 'Cancel' }
        );
        
        if (yes) {
          try {
            await stopAria2();
          } catch (e) {
            console.error("Failed to stop aria2:", e);
          }
          await exit(0);
        }
      });
      return unlisten;
    };

    const unlistenPromise = setupExitHandler();
    return () => {
      unlistenPromise.then(unlisten => unlisten());
    };
  }, []);

  const handleBrowse = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: downloadDir || undefined,
      });
      
      if (selected && typeof selected === "string") {
        setDownloadDir(selected);
      }
    } catch (err) {
      console.error("Failed to open dialog:", err);
    }
  };

  const handleFetch = useCallback(async () => {
    if (!url.trim()) return;
    setIsFetching(true);
    setError(null);
    setTeraboxInfo(null);
    setSelectedFiles(new Set());

    try {
      const info = await getTeraboxInfo(url);
      if (info.ok) {
        setTeraboxInfo(info);
      } else {
        setError(info.error_message || "Failed to fetch file info");
      }
    } catch (err: any) {
      setError(typeof err === "string" ? err : err.message || JSON.stringify(err));
    } finally {
      setIsFetching(false);
    }
  }, [url]);

  const handleDownloadSelected = () => {
    if (!teraboxInfo) return;
    const filesToDownload = teraboxInfo.list.filter(f => selectedFiles.has(f.fs_id) && !f.is_dir);
    
    addToQueue(filesToDownload, {
      shareid: teraboxInfo.shareid,
      uk: teraboxInfo.uk,
      sign: teraboxInfo.sign,
      timestamp: teraboxInfo.timestamp,
      mode: downloadMode,
    }, downloadDir || undefined);
    
    startQueue();
    setSelectedFiles(new Set());
  };

  return (
    <div className="h-screen flex flex-col bg-background text-foreground overflow-hidden">
      {/* Header */}
      <header className="border-b px-6 py-3 flex items-center justify-between bg-card">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
            <Download className="text-primary-foreground h-5 w-5" />
          </div>
          <h1 className="font-bold text-xl">Trauso <span className="text-primary font-normal">v2</span></h1>
        </div>
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="icon" onClick={() => setShowSettings(true)}>
            <Settings className="h-5 w-5" />
          </Button>
        </div>
      </header>

      {/* Main Content Grid */}
      <main className="flex-1 grid grid-cols-12 gap-6 p-6 overflow-hidden">
        
        {/* Left Column: Source (4 cols) */}
        <div className="col-span-12 md:col-span-5 lg:col-span-4 flex flex-col gap-4 overflow-hidden">
          {/* Fetch Card */}
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-muted-foreground uppercase tracking-wider">
                Source URL
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="flex gap-2">
                <div className="relative flex-1">
                  <Link className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
                  <Input 
                    placeholder="https://terabox.com/s/..." 
                    className="pl-9"
                    value={url}
                    onChange={e => setUrl(e.target.value)}
                    onKeyDown={e => e.key === 'Enter' && handleFetch()}
                  />
                </div>
                <Button onClick={handleFetch} disabled={isFetching || !url}>
                  {isFetching ? <Loader2 className="h-4 w-4 animate-spin" /> : "Fetch"}
                </Button>
              </div>
              {error && (
                <div className="mt-2 text-xs text-red-500 bg-red-500/10 p-2 rounded">
                  {error}
                </div>
              )}
            </CardContent>
          </Card>

          {/* File List Card */}
          <Card className="flex-1 flex flex-col overflow-hidden">
             <CardHeader className="pb-3 border-b flex flex-row items-center justify-between">
              <CardTitle className="text-sm font-medium text-muted-foreground uppercase tracking-wider">
                Files {teraboxInfo && `(${teraboxInfo.list.length})`}
              </CardTitle>
              {teraboxInfo && (
                <SelectAllButton 
                  files={teraboxInfo.list} 
                  selectedFiles={selectedFiles} 
                  onSelectionChange={setSelectedFiles} 
                />
              )}
            </CardHeader>
            <CardContent className="flex-1 overflow-y-auto p-2">
              {teraboxInfo ? (
                <FileList 
                  files={teraboxInfo.list} 
                  selectedFiles={selectedFiles} 
                  onSelectionChange={setSelectedFiles} 
                />
              ) : (
                <div className="h-full flex flex-col items-center justify-center text-muted-foreground opacity-50">
                  <FolderOpen className="h-12 w-12 mb-2" />
                  <p>No files loaded</p>
                </div>
              )}
            </CardContent>
            <div className="p-4 border-t bg-muted/20">
              <Button 
                className="w-full" 
                onClick={handleDownloadSelected}
                disabled={selectedFiles.size === 0}
              >
                <FileDown className="mr-2 h-4 w-4" />
                Add to Queue ({selectedFiles.size})
              </Button>
            </div>
          </Card>
        </div>

        {/* Right Column: Queue (8 cols) */}
        <div className="col-span-12 md:col-span-7 lg:col-span-8 flex flex-col overflow-hidden">
          <Card className="h-full flex flex-col overflow-hidden border-2 border-dashed border-muted/50">
            <CardHeader className="pb-3 border-b flex flex-row items-center justify-between bg-muted/10">
              <div className="flex items-center gap-2">
                <CardTitle className="text-sm font-medium uppercase tracking-wider">
                  Download Queue
                </CardTitle>
                <span className="text-xs bg-primary/10 text-primary px-2 py-0.5 rounded-full font-bold">
                  {queue.length}
                </span>
                <span className="text-xs text-muted-foreground ml-2">
                  ({maxConcurrent === 1 ? 'Sequential' : 'Parallel'})
                </span>
              </div>
              <div className="flex items-center gap-1">
                <Button 
                  variant="outline" 
                  size="sm" 
                  onClick={isProcessing ? pauseQueue : startQueue}
                  disabled={queue.length === 0}
                  className="mr-1"
                >
                  {isProcessing ? (
                    <><Pause className="mr-2 h-3 w-3" /> Pause</>
                  ) : (
                    <><Play className="mr-2 h-3 w-3" /> Start</>
                  )}
                </Button>
                <Button variant="ghost" size="icon" onClick={clearCompleted} title="Clear Completed">
                  <Trash2 className="h-4 w-4 text-muted-foreground hover:text-green-600" />
                </Button>
                <Button variant="ghost" size="icon" onClick={clearAll} title="Clear All Queue (Pending & Error)">
                  <ListX className="h-4 w-4 text-muted-foreground hover:text-red-600" />
                </Button>
              </div>
            </CardHeader>
            <CardContent className="flex-1 overflow-y-auto p-0">
              {queue.length === 0 ? (
                <div className="h-full flex flex-col items-center justify-center text-muted-foreground opacity-30">
                  <FileDown className="h-16 w-16 mb-4" />
                  <p className="text-lg font-medium">Queue is empty</p>
                  <p className="text-sm">Add files from the left panel to start downloading</p>
                </div>
              ) : (
                <div className="divide-y">
                  {queue.map(item => (
                    <QueueItemRow 
                      key={item.id} 
                      item={item} 
                      isActive={item.status === 'downloading' || item.status === 'fetching_link'}
                      onRemove={() => removeFromQueue(item.id)}
                    />
                  ))}
                </div>
              )}
            </CardContent>
            {/* Footer Stats */}
            {queue.length > 0 && (
              <div className="p-3 bg-muted/30 border-t text-xs text-muted-foreground flex justify-between">
                <span>{queue.filter(i => i.status === 'completed').length} Completed</span>
                <span>{queue.filter(i => i.status === 'pending').length} Pending</span>
              </div>
            )}
          </Card>
        </div>

      </main>

      {/* Settings Modal */}
      {showSettings && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
          <div className="bg-background border rounded-lg shadow-lg w-[450px] p-6 space-y-6 animate-in fade-in zoom-in-95 duration-200">
            <div className="flex items-center justify-between border-b pb-4">
              <h2 className="text-lg font-semibold flex items-center gap-2">
                <Settings className="h-5 w-5" /> Settings
              </h2>
              <Button variant="ghost" size="icon" onClick={() => setShowSettings(false)}>
                <X className="h-4 w-4" />
              </Button>
            </div>

            <div className="space-y-6">
              <div className="space-y-2">
                <label className="text-sm font-medium">Download Location</label>
                <div className="flex gap-2">
                  <Input 
                    value={downloadDir} 
                    onChange={e => setDownloadDir(e.target.value)} 
                    placeholder="Default: Downloads folder"
                    className="flex-1"
                  />
                  <Button variant="outline" size="icon" onClick={handleBrowse}>
                    <FolderOpen className="h-4 w-4" />
                  </Button>
                </div>
                <p className="text-xs text-muted-foreground">
                  Where files will be saved. Leave empty for default.
                </p>
              </div>

              <div className="space-y-2">
                <label className="text-sm font-medium">Download Server</label>
                <div className="grid grid-cols-2 gap-4">
                    <label className={cn(
                      "flex items-center justify-center gap-2 p-3 rounded-md border cursor-pointer hover:bg-accent transition-all",
                      downloadMode === 1 && "border-primary bg-primary/5"
                    )}>
                        <input type="radio" className="hidden" checked={downloadMode === 1} onChange={() => setDownloadMode(1)} />
                        <span>Server 1</span>
                    </label>
                    <label className={cn(
                      "flex items-center justify-center gap-2 p-3 rounded-md border cursor-pointer hover:bg-accent transition-all",
                      downloadMode === 2 && "border-primary bg-primary/5"
                    )}>
                        <input type="radio" className="hidden" checked={downloadMode === 2} onChange={() => setDownloadMode(2)} />
                        <span>Server 2 (Default)</span>
                    </label>
                </div>
              </div>

              <div className="space-y-2">
                <label className="text-sm font-medium">Download Mode</label>
                <div className="grid grid-cols-2 gap-4">
                    <label className={cn(
                      "flex flex-col items-center justify-center gap-1 p-3 rounded-md border cursor-pointer hover:bg-accent transition-all text-center h-20",
                      maxConcurrent === 1 && "border-primary bg-primary/5"
                    )}>
                        <input 
                            type="radio" 
                            className="hidden"
                            checked={maxConcurrent === 1} 
                            onChange={() => setMaxConcurrent(1)} 
                        />
                        <span className="font-medium">Sequential</span>
                        <span className="text-xs text-muted-foreground">1 file at a time</span>
                    </label>
                    <label className={cn(
                      "flex flex-col items-center justify-center gap-1 p-3 rounded-md border cursor-pointer hover:bg-accent transition-all text-center h-20",
                      maxConcurrent > 1 && "border-primary bg-primary/5"
                    )}>
                        <input 
                            type="radio" 
                            className="hidden"
                            checked={maxConcurrent > 1} 
                            onChange={() => setMaxConcurrent(5)} 
                        />
                        <span className="font-medium">Parallel</span>
                        <span className="text-xs text-muted-foreground">Up to 5 files</span>
                    </label>
                </div>
              </div>
            </div>
            
            <div className="flex justify-end pt-2">
                <Button onClick={handleSaveSettings}>Save & Close</Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function QueueItemRow({ item, isActive, onRemove }: { item: QueueItem, isActive: boolean, onRemove: () => void }) {
  return (
    <div className={cn(
      "p-4 flex items-center gap-4 transition-colors group",
      isActive ? "bg-primary/5" : "hover:bg-accent/50",
      item.status === 'completed' && "opacity-60 bg-muted/20"
    )}>
      <div className={cn(
        "w-2 h-12 rounded-full",
        item.status === 'completed' ? "bg-green-500" :
        item.status === 'error' ? "bg-red-500" :
        isActive ? "bg-primary animate-pulse" : "bg-muted"
      )} />
      
      <div className="flex-1 min-w-0">
        <div className="flex justify-between mb-1">
          <span className="font-medium truncate">{item.file.name}</span>
          <div className="flex items-center gap-2">
            <span className="text-xs font-mono text-muted-foreground">
              {getStatusText(item)}
            </span>
            {item.status !== 'completed' && (
              <Button 
                variant="ghost" 
                size="icon" 
                className="h-6 w-6 text-muted-foreground hover:text-red-500 -mr-2"
                onClick={(e) => {
                  e.stopPropagation();
                  onRemove();
                }}
                title="Remove / Cancel"
              >
                <X className="h-4 w-4" />
              </Button>
            )}
          </div>
        </div>
        
        <div className="h-2 bg-muted rounded-full overflow-hidden">
          <div 
            className={cn(
              "h-full transition-all duration-500",
              item.status === 'error' ? "bg-red-500" : 
              item.status === 'completed' ? "bg-green-500" : "bg-primary"
            )}
            style={{ width: `${item.progress}%` }}
          />
        </div>
        
        <div className="flex justify-between mt-1 text-xs text-muted-foreground">
          <span>{formatBytes(item.downloaded)} / {formatBytes(item.total)}</span>
          {isActive && <span>{formatBytes(item.speed)}/s</span>}
        </div>
        
        {item.error && (
          <p className="text-xs text-red-500 mt-1">{item.error}</p>
        )}
      </div>
    </div>
  );
}

function getStatusText(item: QueueItem) {
  switch (item.status) {
    case 'pending': return 'Wait';
    case 'fetching_link': return 'Getting Link...';
    case 'downloading': return `${item.progress.toFixed(1)}%`;
    case 'completed': return 'Done';
    case 'error': return 'Failed';
  }
}
