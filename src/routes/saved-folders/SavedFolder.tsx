import { Button, Group, Stack, Text } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { memo } from "react";
import { DirWithFiles } from "./FilesInDirs.type";

interface Props {
  dir: DirWithFiles;
}

const SavedDir = ({ dir }: Props) => {
  const removeFolderMut = useMutation({
    mutationKey: ["remove_saved_folder"],
    mutationFn: async (path: string) =>
      await invoke("remove_dir", { dir: path }),
    meta: { invalidateQueryKey: ["get_files_in_dirs"] }
  });

  const rescanDir = useMutation({
    mutationKey: ["rescan_dir"],
    mutationFn: async (path: string) =>
      await invoke("rescan_dir", { dir: path }),
    meta: { invalidateQueryKey: ["get_files_in_dirs"] }
  });

  const generateThumbs = useMutation({
    mutationKey: ["generate_thumbnails"],
    mutationFn: async (path: string) =>
      await invoke("generate_thumbnails", { dir: path })
  });

  const disabled =
    removeFolderMut.isPending ||
    rescanDir.isPending ||
    generateThumbs.isPending;

  return (
    <Group wrap="nowrap">
      <Stack gap="0">
        <Text w="100%" style={{ wordBreak: "break-word", userSelect: "text" }}>
          {dir.path}
        </Text>
        <Text size="sm" c="gray" style={{ userSelect: "text" }}>
          {dir.files.length} files
        </Text>
      </Stack>

      <Group wrap="nowrap" ml="auto" style={{ flexShrink: 0 }}>
        <Button
          variant="outline"
          onClick={() => generateThumbs.mutate(dir.path)}
          disabled={disabled}
        >
          Generate thumbnails
        </Button>
        <Button
          variant="outline"
          onClick={() => rescanDir.mutate(dir.path)}
          disabled={disabled}
        >
          Sync
        </Button>
        <Button
          color="red"
          variant="outline"
          onClick={() => removeFolderMut.mutate(dir.path)}
          disabled={disabled}
        >
          Delete
        </Button>
      </Group>
    </Group>
  );
};

export default memo(SavedDir);
