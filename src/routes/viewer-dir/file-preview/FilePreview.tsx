import { Button, Stack, Text } from "@mantine/core";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import css from "./FilePreview.module.css";
import { path } from "@tauri-apps/api";
import { use } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

const cacheDirPromise = path.appCacheDir();

type Props = {
  file: DirWithFiles["files"][number] | null;
  onClose: () => void;
};

const FilePreview = ({ file, onClose }: Props) => {
  const cacheDir = use(cacheDirPromise);
  const src = convertFileSrc(`${cacheDir}\\files\\${file?.id}\\thumbnail.avif`);

  return (
    <div className={css.filePreview}>
      <Stack>
        <Button onClick={onClose}>Close</Button>

        <img alt={file?.name} src={src} />

        <Text>{file?.name}</Text>

        <pre>{JSON.stringify(file, null, 2)}</pre>
      </Stack>
    </div>
  );
};

export default FilePreview;
