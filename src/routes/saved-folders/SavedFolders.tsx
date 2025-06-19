import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import Layout from "../../components/Layout";
import AddSavedFolder from "./AddSavedFolder";
import SavedFolder from "./SavedFolder";
import { useDirs } from "./useDirs";

const SavedDirs = () => {
  const dirsQuery = useDirs();

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
