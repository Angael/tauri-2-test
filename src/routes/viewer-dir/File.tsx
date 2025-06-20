import { memo, useMemo } from "react";
import css from "./File.module.css";
import { convertFileSrc } from "@tauri-apps/api/core";
import { useInViewport } from "@mantine/hooks";
import { DirWithFiles } from "../saved-folders/FilesInDirs.type";

type Props = {
  dir: string;
  file: DirWithFiles["files"][number];
};

const File = ({ dir, file }: Props) => {
  const { ref, inViewport } = useInViewport();

  const src = useMemo(() => {
    // TODO: works only on Windows, and bad performance?

    // When fetching prepare the thumbnails, and join them with
    // import { join } from "@tauri-apps/api/path";
    return convertFileSrc(dir + "\\" + file.name);
  }, [dir, file.name]);

  return (
    <div ref={ref} className={css.fileWrapper}>
      {file.name}
      {inViewport && (
        <img
          src={src}
          alt={file.name}
          key={file.name}
          style={{
            width: "200px",
            height: "100px",
            objectFit: "cover",
            margin: "5px"
          }}
        />
      )}
    </div>
  );
};

export default memo(File);
