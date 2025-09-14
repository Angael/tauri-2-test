import { Button } from "@mantine/core";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import css from "./FilePreview.module.css";
import { convertFileSrc } from "@tauri-apps/api/core";
import { env } from "../../../util/env";

type Props = {
  dirPath: string;
  file: DirWithFiles["files"][number] | null;
  onClose: () => void;
};

const FilePreview = ({ dirPath, file, onClose }: Props) => {
  const src = convertFileSrc(dirPath + "\\" + file?.name);

  const isVideo = file?.name.match(/\.(mp4|mov|avi|mkv|webm)$/i);

  return (
    <div className={css.filePreview}>
      <Button className={css.closeBtn} onClick={onClose}>
        Close
      </Button>

      {isVideo ? (
        <video className={css.image} controls src={src} muted autoPlay />
      ) : (
        <img className={css.image} alt={file?.name} src={src} />
      )}

      {!env.isProd && (
        <details className={css.debug}>
          <summary>{file?.name} (JSON)</summary>
          <pre>{JSON.stringify(file, null, 2)}</pre>
        </details>
      )}
    </div>
  );
};

export default FilePreview;
