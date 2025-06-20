import { useInViewport } from "@mantine/hooks";
import { memo } from "react";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import css from "./File.module.css";
import FilePlaceholder from "./FilePlaceholder";

type Props = {
  dir: string;
  file: DirWithFiles["files"][number];
};

const File = ({ dir, file }: Props) => {
  const { ref, inViewport } = useInViewport();

  // const src = useMemo(() => {
  //   // TODO: works only on Windows, and bad performance?

  //   // When fetching prepare the thumbnails, and join them with
  //   // import { join } from "@tauri-apps/api/path";
  //   return convertFileSrc(dir + "\\" + file.name);
  // }, [dir, file.name]);

  return (
    <div ref={ref} className={css.fileWrapper}>
      {inViewport && <FilePlaceholder file={file} />}
    </div>
  );
};

export default memo(File);
