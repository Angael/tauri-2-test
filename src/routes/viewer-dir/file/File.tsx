import { useInViewport } from "@mantine/hooks";
import { memo, use } from "react";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import css from "./File.module.css";
import FilePlaceholder from "./FilePlaceholder";
import { convertFileSrc } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";

type Props = {
  dir: string;
  file: DirWithFiles["files"][number];
};

const File = ({ dir: _, file }: Props) => {
  const { ref, inViewport } = useInViewport();

  // const src = useMemo(() => {
  //   // TODO: works only on Windows, and bad performance?

  //   // When fetching prepare the thumbnails, and join them with
  //   // import { join } from "@tauri-apps/api/path";
  //   return convertFileSrc(dir + "\\" + file.name);
  // }, [dir, file.name]);

  // const hasThumbnail = file;

  // useEffect(() => {
  //   const unlistenPromise = listen<TaskGenerateThumbEvent>(
  //     "task_generate_thumb",
  //     (event) => {
  //       console.log("Event processed:", event);
  //       // if (event.payload.dir === dir.path) {
  //       //   setGeneratedThumbs((prev) => prev + 1);
  //       // }
  //     }
  //   );

  //   return () => {
  //     unlistenPromise.then((f) => f());
  //   };
  // }, []);

  // TODO: Remove hardcoded path
  const src = convertFileSrc(
    `C:\\Users\\krzys\\AppData\\Local\\com.tauri-2-test.app\\files\\${file.id}\\thumbnail.avif`
  );

  return (
    <div ref={ref} className={css.fileWrapper}>
      {/* {inViewport && <FilePlaceholder file={file} />} */}
      {inViewport && (
        <img src={src} alt={file.name} style={{ width: "100%" }} />
      )}
    </div>
  );
};

export default memo(File);
