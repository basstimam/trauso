import { load } from '@tauri-apps/plugin-store';

const STORE_PATH = 'settings.json';

export interface AppSettings {
  downloadDir: string;
  downloadMode: number;
  maxConcurrent: number;
}

export const DEFAULT_SETTINGS: AppSettings = {
  downloadDir: '',
  downloadMode: 2,
  maxConcurrent: 1,
};

let storePromise: Promise<any> | null = null;

function getStore() {
  if (!storePromise) {
    storePromise = load(STORE_PATH);
  }
  return storePromise;
}

export async function loadSettings(): Promise<AppSettings> {
  try {
    const s = await getStore();
    const val = await s.get('config');
    return { ...DEFAULT_SETTINGS, ...val };
  } catch (e) {
    console.error("Failed to load settings:", e);
    return DEFAULT_SETTINGS;
  }
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  try {
    const s = await getStore();
    await s.set('config', settings);
    await s.save();
  } catch (e) {
    console.error("Failed to save settings:", e);
  }
}
