import { create } from "zustand";
import { DirWithFiles } from "../routes/saved-folders/FilesInDirs.type";

interface CurrentPreviewState {
  isOpen: boolean;
  file: DirWithFiles["files"][number] | null;

  set: (newValue: CurrentPreviewState["file"]) => void;
  close: () => void;
}

export const currentPreview$ = create<CurrentPreviewState>((set) => ({
  isOpen: false,
  file: null,

  set(newValue) {
    return set({ isOpen: true, file: newValue });
  },

  close() {
    return set({ isOpen: false });
  }
}));
