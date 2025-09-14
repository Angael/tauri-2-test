import { useInViewport } from "@mantine/hooks";
import { memo } from "react";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import css from "./File.module.css";
import FilePlaceholder from "./FilePlaceholder";
import Thumbnail from "./Thumbnail";
import { useStore } from "zustand";
import { currentPreview$ } from "../../../stores/currentPreview$";

type Props = {
  dir: string;
  file: DirWithFiles["files"][number];
};

const File = ({ dir: _, file }: Props) => {
  const { ref, inViewport } = useInViewport();
  const setPreviewitem = useStore(currentPreview$, (s) => s.set);

  const content =
    file.thumbs.length > 0 ? (
      <Thumbnail file={file} />
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
