import { platform } from "@tauri-apps/plugin-os";

const currentPlatform = platform();

export const pathSlash = currentPlatform === "windows" ? "\\" : "/";

export const joinPath = (...parts: string[]) => parts.join(pathSlash);
