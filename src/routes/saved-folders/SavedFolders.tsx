import { Button, Group, Stack, Text } from "@mantine/core";
import { useMutation, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import Layout from "../../components/Layout";
import AddSavedFolder from "./AddSavedFolder";

interface File {
  name: string;
  size: number;
}

interface DirWithFiles {
  path: string;
  files: File[];
}

interface FilesInDirs {
  dirs: Array<DirWithFiles>;
}

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

  const removeFolderMut = useMutation({
    mutationKey: ["remove_saved_folder"],
    mutationFn: async (path: string) =>
      await invoke("remove_dir", { dir: path }),
    onSuccess: () => {
      dirsQuery.refetch();
    }
  });

  const rescanDir = useMutation({
    mutationKey: ["rescan_dir"],
    mutationFn: async (path: string) =>
      await invoke("rescan_dir", { dir: path }),
    onSuccess: () => {
      dirsQuery.refetch();
    }
  });

  const disableButtons =
    dirsQuery.isPending ||
    addFolderMut.isPending ||
    removeFolderMut.isPending ||
    rescanDir.isPending;

  return (
    <Layout>
      <AddSavedFolder
        onAddFolder={addFolderMut.mutate}
        isPending={addFolderMut.isPending}
      />
      {dirsQuery.data?.dirs.map((dir, index) => (
        <Group key={index} wrap="nowrap">
          <Stack gap="0">
            <Text
              w="100%"
              style={{ wordBreak: "break-word", userSelect: "text" }}
            >
              {dir.path}
            </Text>
            <Text size="sm" c="gray" style={{ userSelect: "text" }}>
              {dir.files.length} files
            </Text>
          </Stack>

          <Group wrap="nowrap" ml="auto" style={{ flexShrink: 0 }}>
            <Button
              variant="outline"
              onClick={() => rescanDir.mutate(dir.path)}
              disabled={disableButtons}
            >
              Sync
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
