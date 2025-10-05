import { useInViewport } from "@mantine/hooks";
import { memo, useMemo } from "react";
import { useStore } from "zustand";
import { currentPreview$ } from "../../../stores/currentPreview$";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import css from "./File.module.css";
import Thumbnail from "./Thumbnail";
import { getFileType } from "../../../util/util";
import FilePlaceholder from "./FilePlaceholder";

type Props = {
  dir: string;
  file: DirWithFiles["files"][number];
};

const File = ({ dir, file }: Props) => {
  const { ref, inViewport } = useInViewport();
  const setPreviewitem = useStore(currentPreview$, (s) => s.set);

  const fileType = useMemo(() => getFileType(file.name), [file.name]);

  const content =
    file.thumbs.length > 0 || fileType === "image" ? (
      <Thumbnail file={file} dir={dir} />
    ) : (
      <FilePlaceholder file={file} />
    );

  const onClick = () => {
    console.log("click", file);
    setPreviewitem(file);
  };

  return (
    <div ref={ref} className={css.fileWrapper} onMouseDown={onClick}>
      {inViewport && content}
    </div>
  );
};

export default memo(File);
