import { ActionIcon, Card, Menu, Progress, Stack, Text } from "@mantine/core";
import { mdiMenu, mdiSync, mdiTrashCan } from "@mdi/js";
import Icon from "@mdi/react";
import { useMutation } from "@tanstack/react-query";
import { path } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import numeral from "numeral";
import { memo, use, useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router";
import { DirScanProgressEvent } from "../../types/tauriEvent.type";
import { DirWithFiles } from "./FilesInDirs.type";
import { getPreviewThumbsForSavedFolder } from "./getPreviewThumbsForSavedFolder";
import css from "./SavedFolder.module.css";
import { splitPathAndFilename } from "../../util/splitPathAndFilename";

const cacheDirPromise = path.appCacheDir();

interface Props {
  dir: DirWithFiles;
}

const SavedDir = ({ dir }: Props) => {
  // TODO this is bad, because when it unmounts, progress looks reset
  // TBH the only progression is to mark items as processed, somewhere in tauri state :(
  const [processedElements, setProcessedElements] = useState<number[]>([]);
  const cacheDir = use(cacheDirPromise);

  const resetProcessedElements = () => setProcessedElements([]);

  const stats = useMemo(() => {
    return dir.files.reduce(
      (acc, file) => {
        acc.totalSize += file.size;
        return acc;
      },
      { totalSize: 0 }
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
    const unlistenPromise = listen<DirScanProgressEvent>(
      "dir_scan_progress",
      (event) => {
        if (event.payload.dir === dir.path) {
          console.log("Event processed:", event);
          setProcessedElements((prev) => [...prev, event.payload.i]);
        }
      }
    );

    return () => {
      unlistenPromise.then((f) => f());
    };
  }, []);

  const [path, dirName] = splitPathAndFilename(dir.path);

  const firstFewThumbnails = useMemo(
    () => getPreviewThumbsForSavedFolder(cacheDir, dir, 8),
    [cacheDir, dir]
  );

  return (
    <Card className={css.savedFolder} shadow="md" withBorder onClick={onOpen}>
      <div className={css.thumbs}>
        {firstFewThumbnails.map((thumbSrc, i) => (
          <img key={i} src={thumbSrc} className={css.thumb} />
        ))}
      </div>

      <div className={css.cardBody}>
        <Stack gap="0" style={{ userSelect: "text" }}>
          <Text c="gray" size="xs" w="100%" style={{ wordBreak: "break-word" }}>
            {path}
          </Text>
          <Text w="100%" style={{ wordBreak: "break-word" }}>
            {dirName}
          </Text>
          <Text size="sm" c="gray">
            {dir.files.length} files, {numeral(stats.totalSize).format("0.00b")}
          </Text>
          {processedElements.length > 0 && (
            <Progress.Root size="xl">
              <Progress.Section
                value={(100 * processedElements.length) / dir.files.length}
              >
                <Progress.Label>Prepared files</Progress.Label>
              </Progress.Section>
            </Progress.Root>
          )}
        </Stack>

        <Menu>
          <Menu.Target>
            <ActionIcon
              variant="light"
              size="lg"
              onClick={(e) => e.stopPropagation()}
              title="Menu"
              style={{ marginLeft: "auto" }}
            >
              <Icon path={mdiMenu} size={1} />
            </ActionIcon>
          </Menu.Target>

          <Menu.Dropdown>
            <Menu.Item
              leftSection={<Icon path={mdiSync} size={1} />}
              onClick={(e) => {
                e.stopPropagation();
                resetProcessedElements();
                generateThumbs.mutate(dir.path);
              }}
              disabled={disabled}
            >
              Regenerate thumbnails
            </Menu.Item>

            <Menu.Item
              leftSection={<Icon path={mdiSync} size={1} />}
              onClick={(e) => {
                e.stopPropagation();
                resetProcessedElements();
                rescanDir.mutate(dir.path);
              }}
              disabled={disabled}
            >
              Re-sync
            </Menu.Item>

            <Menu.Divider />

            <Menu.Item
              color="red"
              leftSection={<Icon path={mdiTrashCan} size={1} />}
              onClick={(e) => {
                e.stopPropagation();
                removeFolderMut.mutate(dir.path);
              }}
            >
              Stop optimizing directory
            </Menu.Item>
          </Menu.Dropdown>
        </Menu>
      </div>
    </Card>
  );
};

export default memo(SavedDir);
