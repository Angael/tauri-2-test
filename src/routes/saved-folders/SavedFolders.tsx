import { useMutation, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import Layout from "../../components/Layout";
import AddSavedFolder from "./AddSavedFolder";
import { FilesInDirs } from "./FilesInDirs.type";
import SavedFolder from "./SavedFolder";

const SavedDirs = () => {
  const dirsQuery = useQuery({
    queryKey: ["get_files_in_dirs"],
    queryFn: async () => {
      return await invoke<FilesInDirs>("get_files_in_dirs");
    }
  });

  const addFolderMut = useMutation({
    mutationKey: ["add_dir"],
    mutationFn: async (path: string) => invoke("add_dir", { dir: path }),
    onSuccess: () => {
      dirsQuery.refetch();
    }
  });

  return (
    <Layout>
      <AddSavedFolder
        onAddFolder={addFolderMut.mutate}
        isPending={addFolderMut.isPending}
      />
      {dirsQuery.data?.dirs.map((dir) => (
        <SavedFolder key={dir.path} dir={dir} />
      ))}
    </Layout>
  );
};

export default SavedDirs;
