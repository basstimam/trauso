import {
  File,
  Folder,
  Video,
  Image,
  FileArchive,
  Check,
} from "lucide-react";
import { cn } from "../lib/utils";
import type { TeraboxFileInfo } from "../lib/types";
import { formatBytes } from "../lib/api";

interface FileListProps {
  files: TeraboxFileInfo[];
  selectedFiles: Set<string>;
  onSelectionChange: (selected: Set<string>) => void;
}

export function FileList({
  files,
  selectedFiles,
  onSelectionChange,
}: FileListProps) {
  if (files.length === 0) {
    return (
      <div className="text-center py-8 text-muted-foreground">
        No files found
      </div>
    );
  }

  return (
    <div className="space-y-1">
      {files.map((file) => (
        <FileItem
          key={file.fs_id}
          file={file}
          selectedFiles={selectedFiles}
          onSelectionChange={onSelectionChange}
        />
      ))}
    </div>
  );
}

interface FileItemProps {
  file: TeraboxFileInfo;
  selectedFiles: Set<string>;
  onSelectionChange: (selected: Set<string>) => void;
}

function FileItem({
  file,
  selectedFiles,
  onSelectionChange,
}: FileItemProps) {
  const isSelected = selectedFiles.has(file.fs_id);

  const toggleSelection = () => {
    const newSelected = new Set(selectedFiles);
    if (isSelected) {
      newSelected.delete(file.fs_id);
    } else {
      newSelected.add(file.fs_id);
    }
    onSelectionChange(newSelected);
  };

  const getFileIcon = () => {
    if (file.is_dir) return <Folder className="h-4 w-4 text-yellow-500" />;

    switch (file.file_type) {
      case "video":
        return <Video className="h-4 w-4 text-blue-500" />;
      case "image":
        return <Image className="h-4 w-4 text-green-500" />;
      case "file":
        return <FileArchive className="h-4 w-4 text-purple-500" />;
      default:
        return <File className="h-4 w-4 text-gray-500" />;
    }
  };

  return (
    <div
      className={cn(
        "flex items-center gap-2 py-2 px-3 rounded-md hover:bg-accent cursor-pointer transition-colors",
        isSelected && "bg-accent"
      )}
    >
      <button
        onClick={toggleSelection}
        className={cn(
          "flex items-center justify-center w-5 h-5 rounded border-2 transition-colors",
          isSelected
            ? "bg-primary border-primary text-primary-foreground"
            : "border-muted-foreground/50 hover:border-primary"
        )}
      >
        {isSelected && <Check className="h-3 w-3" />}
      </button>

      <div className="flex items-center gap-2 flex-1" onClick={toggleSelection}>
        {getFileIcon()}
        <span className="flex-1 truncate">{file.name}</span>
        {!file.is_dir && file.size && (
          <span className="text-sm text-muted-foreground">
            {formatBytes(file.size)}
          </span>
        )}
      </div>
    </div>
  );
}

export function SelectAllButton({
  files,
  selectedFiles,
  onSelectionChange,
}: FileListProps) {
  const allIds = files.map((file) => file.fs_id);
  const allSelected = allIds.length > 0 && allIds.every((id) => selectedFiles.has(id));

  const toggleSelectAll = () => {
    if (allSelected) {
      onSelectionChange(new Set());
    } else {
      onSelectionChange(new Set(allIds));
    }
  };

  return (
    <button
      onClick={toggleSelectAll}
      className={cn(
        "flex items-center gap-2 px-3 py-1.5 rounded-md text-sm transition-colors",
        allSelected
          ? "bg-primary text-primary-foreground"
          : "bg-muted hover:bg-muted/80"
      )}
    >
      {allSelected ? "Deselect All" : "Select All"}
    </button>
  );
}
