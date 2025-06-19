import { useQuery } from "@tanstack/react-query";
import { FilesInDirs } from "./FilesInDirs.type";
import { invoke } from "@tauri-apps/api/core";

export const useDirs = () =>
  useQuery({
    queryKey: ["get_files_in_dirs"],
    queryFn: async () => {
      return await invoke<FilesInDirs>("get_files_in_dirs");
    }
  });
