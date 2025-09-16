import { Button } from "@mantine/core";
import { convertFileSrc } from "@tauri-apps/api/core";
import { ComponentPropsWithRef, memo } from "react";
import { useStore } from "zustand";
import { currentPreview$ } from "../../../stores/currentPreview$";
import { env } from "../../../util/env";
import css from "./FilePreview.module.css";

type Props = {
  dirPath: string;
} & ComponentPropsWithRef<"div">;

const FilePreview = ({ dirPath, ...props }: Props) => {
  const file = useStore(currentPreview$, (s) => s.file);
  const src = convertFileSrc(dirPath + "\\" + file?.name);
  const onClose = useStore(currentPreview$, (s) => s.close);

  const isVideo = file?.name.match(/\.(mp4|mov|avi|mkv|webm)$/i);

  return (
    <div className={css.filePreview} {...props}>
      <Button className={css.closeBtn} onClick={onClose}>
        Close
      </Button>

      {file && (
        <>
          {isVideo ? (
            <video
              key={file.id ?? "nofile"}
              className={css.video}
              controls
              src={src}
              muted
              autoPlay
              loop
            />
          ) : (
            <img className={css.image} alt={file.name} src={src} />
          )}
        </>
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

export default memo(FilePreview);
