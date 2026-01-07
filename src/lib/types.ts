export interface TeraboxFileInfo {
  is_dir: boolean;
  fs_id: string;
  name: string;
  file_type: "video" | "image" | "file" | "folder" | "other";
  size: number | null;
  category: string | null;
  create_time: number | null;
}

export interface TeraboxInfo {
  ok: boolean;
  shareid: number;
  uk: number;
  sign: string;
  timestamp: number;
  list: TeraboxFileInfo[];
  error_message?: string;
}

export interface DownloadParams {
  shareid: number;
  uk: number;
  sign: string;
  timestamp: number;
  fs_id: string;
  mode: number;
}

export interface DownloadLink {
  ok: boolean;
  download_link: string | null;
  error_message?: string;
}

export interface DownloadInfo {
  gid: string;
  filename: string;
  total_size: number;
  downloaded: number;
  speed: number;
  progress: number;
  status: "active" | "waiting" | "paused" | "complete" | "error" | "removed";
  error_message: string | null;
}

export type DownloadStatus = DownloadInfo["status"];
