import { Button } from "@mantine/core";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-fs";
import { ComponentPropsWithRef, memo, useEffect, useState } from "react";
import { useStore } from "zustand";
import { currentPreview$ } from "../../../stores/currentPreview$";
import { env } from "../../../util/env";
import { joinPath } from "../../../util/pathSlash";
import { getFileType } from "../../../util/util";
import css from "./FilePreview.module.css";
import { PreviewSrcStore } from "../../../stores/previewSrcStore";

type Props = {
  dirPath: string;
} & ComponentPropsWithRef<"div">;

const FilePreview = ({ dirPath, ...props }: Props) => {
  const file = useStore(currentPreview$, (s) => s.file);
  const _src = convertFileSrc(joinPath(dirPath, file?.name ?? ""));
  const onClose = useStore(currentPreview$, (s) => s.close);

  const { src, status, set, setWithFsFallback } = useStore(PreviewSrcStore);
  console.log({ src, status });

  const isVideo = getFileType(file?.name ?? "") === "video";

  useEffect(() => {
    set(_src);
  }, [_src]);

  return (
    <div className={css.filePreview} {...props}>
      <Button className={css.closeBtn} onClick={onClose}>
        Close
      </Button>

      {file && (
        <>
          {isVideo ? (
            <video
              key={(src ?? "nofile") + status}
              className={css.video}
              controls
              src={src ?? ""}
              onError={(e) => {
                console.log("error loading vid", e);
                setWithFsFallback(_src);
              }}
              muted
              autoPlay
              loop
            />
          ) : (
            <img className={css.image} alt={file.name} src={src ?? ""} />
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
