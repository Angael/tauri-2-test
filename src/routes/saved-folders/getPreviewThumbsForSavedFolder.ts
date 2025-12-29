import { convertFileSrc } from "@tauri-apps/api/core";
import { DirWithFiles } from "./FilesInDirs.type";
import { joinPath } from "../../util/pathSlash";

export const getPreviewThumbsForSavedFolder = (
  cacheDir: string,
  dir: DirWithFiles,
  maxCount = 4
): string[] => {
  const thumbs: string[] = [];
  for (const file of dir.files.reverse()) {
    for (const thumb of file.thumbs) {
      let usable = true; // TODO: determine if usable
      if (usable) {
        thumbs.push(
          convertFileSrc(joinPath(cacheDir, "files", file.id, "thumbnail.avif"))
        );

        // Only one thumbnail per file
        break;
      }
    }
    if (thumbs.length >= maxCount) {
      break;
    }
  }
  return thumbs;
};
