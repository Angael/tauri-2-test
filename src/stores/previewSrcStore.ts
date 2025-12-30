import { create } from "zustand";
import { open } from "@tauri-apps/plugin-fs";

interface PreviewSrcState {
  status: "idle" | "loading" | "error" | "loaded";
  src: string | null;

  set: (newValue: string) => void;
  setWithFsFallback: (newValue: string) => void;
}

export const PreviewSrcStore = create<PreviewSrcState>((set) => ({
  status: "idle",
  src: null,

  set(newValue) {
    console.log("set()", newValue);
    return set((state) => {
      if (state.src) {
        URL.revokeObjectURL(state.src);
      }

      return { status: "loaded", src: newValue };
    });
  },

  async setWithFsFallback(newValue) {
    console.log("setWithFsFallback()", newValue);

    set((state) => {
      if (state.src === newValue) {
        return state;
      }

      if (state.src) {
        URL.revokeObjectURL(state.src);
      }

      return { status: "loading", src: null };
    });

    try {
      const file = await open(newValue, { read: true });

      const stat = await file.stat();
      if (!stat || !stat.size || stat.size > 1024 * 1024 * 1024) {
        throw new Error("File too large");
      }

      const buf = new Uint8Array(stat.size);
      await file.read(buf);
      const blob = new Blob([buf], { type: "video/mp4" });
      const objectUrl = URL.createObjectURL(blob);

      set({ status: "loaded", src: objectUrl });
      await file.close();
    } catch (e) {
      set({ status: "error", src: null });
      return;
    }
  }
}));
