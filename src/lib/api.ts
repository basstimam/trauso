import { invoke } from "@tauri-apps/api/core";
import type {
  TeraboxInfo,
  DownloadParams,
  DownloadLink,
  DownloadInfo,
} from "./types";

export async function getTeraboxInfo(url: string): Promise<TeraboxInfo> {
  return invoke<TeraboxInfo>("get_terabox_info", { url });
}

export async function getDownloadLink(
  params: DownloadParams
): Promise<DownloadLink> {
  return invoke<DownloadLink>("get_download_link", { params });
}

export async function extractShorturl(url: string): Promise<string | null> {
  return invoke<string | null>("extract_shorturl", { url });
}

export async function wrapDownloadUrl(url: string): Promise<string> {
  return invoke<string>("wrap_download_url", { url });
}

export async function startAria2(): Promise<void> {
  return invoke<void>("start_aria2");
}

export async function stopAria2(): Promise<void> {
  return invoke<void>("stop_aria2");
}

export async function isAria2Running(): Promise<boolean> {
  return invoke<boolean>("is_aria2_running");
}

export async function addDownload(
  url: string,
  dir?: string,
  filename?: string
): Promise<string> {
  return invoke<string>("add_download", { url, dir, filename });
}

export async function getDownloadStatus(gid: string): Promise<DownloadInfo> {
  return invoke<DownloadInfo>("get_download_status", { gid });
}

export async function pauseDownload(gid: string): Promise<string> {
  return invoke<string>("pause_download", { gid });
}

export async function resumeDownload(gid: string): Promise<string> {
  return invoke<string>("resume_download", { gid });
}

export async function cancelDownload(gid: string): Promise<string> {
  return invoke<string>("cancel_download", { gid });
}

export async function getAllDownloads(): Promise<DownloadInfo[]> {
  return invoke<DownloadInfo[]>("get_all_downloads");
}

export async function pauseAllDownloads(): Promise<string> {
  return invoke<string>("pause_all_downloads");
}

export async function resumeAllDownloads(): Promise<string> {
  return invoke<string>("resume_all_downloads");
}

export function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
}

export function formatSpeed(bytesPerSecond: number): string {
  return `${formatBytes(bytesPerSecond)}/s`;
}

export function formatEta(totalSize: number, downloaded: number, speed: number): string {
  if (speed === 0) return "∞";
  const remaining = totalSize - downloaded;
  const seconds = Math.floor(remaining / speed);
  
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
  return `${Math.floor(seconds / 3600)}h ${Math.floor((seconds % 3600) / 60)}m`;
}
