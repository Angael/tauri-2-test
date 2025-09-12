import { Button, Stack, Text } from "@mantine/core";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import css from "./FilePreview.module.css";
import { path } from "@tauri-apps/api";
import { use } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

const cacheDirPromise = path.appCacheDir();

type Props = {
  dirPath: string;
  file: DirWithFiles["files"][number] | null;
  onClose: () => void;
};

const FilePreview = ({ dirPath, file, onClose }: Props) => {
  const cacheDir = use(cacheDirPromise);

  const thumbSrc = convertFileSrc(
    `${cacheDir}\\files\\${file?.id}\\thumbnail.avif`
  );
  const src = convertFileSrc(dirPath + "\\" + file?.name);

  return (
    <div className={css.filePreview}>
      {/* <Stack> */}
      <Button className={css.closeBtn} onClick={onClose}>
        Close
      </Button>

      <img
        className={css.image}
        alt={file?.name}
        src={src}
        style={{ backgroundImage: `url(${thumbSrc})` }}
      />

      <Text>{file?.name}</Text>

      <pre>{JSON.stringify(file, null, 2)}</pre>
      {/* </Stack> */}
    </div>
  );
};

export default FilePreview;
