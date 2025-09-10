import { create } from "zustand";
import { DirWithFiles } from "../routes/saved-folders/FilesInDirs.type";

interface CurrentPreviewState {
  file: DirWithFiles["files"][number] | null;

  set: (newValue: CurrentPreviewState["file"]) => void;
}

export const $currentPreview = create<CurrentPreviewState>((set) => ({
  file: null,

  set(newValue) {
    return set({ file: newValue });
  }
}));
