import { Button, Group, Text } from "@mantine/core";
import { useMutation, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import Layout from "../../components/Layout";
import AddSavedFolder from "./AddSavedFolder";

interface SavedFolder {
  path: string;
}

const SavedDirs = () => {
  const savedDirs = useQuery({
    queryKey: ["get_saved_folders"],
    queryFn: async () => {
      const _folders = await invoke<SavedFolder[]>("get_saved_folders");
      return _folders;
    }
    // staleTime: Infinity
  });

  const addFolderMut = useMutation({
    mutationKey: ["save_folders"],
    mutationFn: async (path: string) => {
      if (!savedDirs.data) {
        return;
      }

      const newFolder = { path };

      await invoke("save_folders", {
        savedFolders: [...savedDirs.data, newFolder]
      });
    },
    onSuccess: () => {
      savedDirs.refetch();
    }
  });

  const removeFolderMut = useMutation({
    mutationKey: ["remove_saved_folder"],
    mutationFn: async (path: string) => {
      if (!savedDirs.data) {
        return;
      }

      const updatedFolders = savedDirs.data.filter(
        (folder) => folder.path !== path
      );

      await invoke("save_folders", { savedFolders: updatedFolders });
    },
    onSuccess: () => {
      savedDirs.refetch();
    }
  });

  const disableButtons =
    savedDirs.isPending || addFolderMut.isPending || removeFolderMut.isPending;

  return (
    <Layout>
      <AddSavedFolder
        onAddFolder={addFolderMut.mutate}
        isPending={addFolderMut.isPending}
      />
      {savedDirs.data?.map((dir, index) => (
        <Group key={index}>
          <Text style={{ userSelect: "text" }}>{dir.path}</Text>
          <Button variant="outline" disabled={disableButtons}>
            Edit
          </Button>
          <Button
            color="red"
            variant="outline"
            onClick={() => removeFolderMut.mutate(dir.path)}
            disabled={disableButtons}
          >
            Delete
          </Button>
        </Group>
      ))}
      {/* Output:
      <pre>
        <code>{JSON.stringify(savedDirs.data, null, 2)}</code>
      </pre> */}
    </Layout>
  );
};

export default SavedDirs;
