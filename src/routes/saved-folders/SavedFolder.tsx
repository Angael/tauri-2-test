import { Button, Group, Stack, Text } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import numeral from "numeral";
import { memo, useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router";
import { DirWithFiles, TaskGenerateThumbEvent } from "./FilesInDirs.type";
import { Progress } from "@mantine/core";

interface Props {
  dir: DirWithFiles;
}

const SavedDir = ({ dir }: Props) => {
  const [processing, setProcessing] = useState<false | number>(false);

  const stats = useMemo(() => {
    return dir.files.reduce(
      (acc, file) => {
        acc.videos += file.video_stats ? 1 : 0;
        acc.totalSize += file.size;
        return acc;
      },
      { videos: 0, totalSize: 0 }
    );
  }, [dir.files]);

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

  const navigate = useNavigate();

  const onOpen = () => {
    navigate(`/viewer-dir/${encodeURIComponent(dir.path)}`);
  };

  const disabled =
    removeFolderMut.isPending ||
    rescanDir.isPending ||
    generateThumbs.isPending;

  useEffect(() => {
    const unlistenPromise = listen<TaskGenerateThumbEvent>(
      "task_generate_thumb",
      (event) => {
        console.log("Event processed:", event);
        if (event.payload.dir === dir.path) {
          setProcessing((prev) => (prev || 0) + 1);
        }
      }
    );

    return () => {
      unlistenPromise.then((f) => f());
    };
  }, []);

  return (
    <Group wrap="nowrap">
      <Stack gap="0">
        <Text w="100%" style={{ wordBreak: "break-word", userSelect: "text" }}>
          {dir.path}
        </Text>
        <Text size="sm" c="gray" style={{ userSelect: "text" }}>
          {dir.files.length} files, {numeral(stats.totalSize).format("0.00b")}
        </Text>
        <Text size="sm" c="gray" style={{ userSelect: "text" }}>
          {stats.videos} videos
        </Text>
        {processing && (
          <Progress.Root size="xl">
            <Progress.Section value={processing ?? 0}>
              <Progress.Label>Prepared files</Progress.Label>
            </Progress.Section>
          </Progress.Root>
        )}
      </Stack>

      <Group wrap="nowrap" ml="auto" style={{ flexShrink: 0 }}>
        <Button onClick={onOpen} disabled={disabled}>
          Open
        </Button>
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
