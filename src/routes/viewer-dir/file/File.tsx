import { useInViewport } from "@mantine/hooks";
import { convertFileSrc } from "@tauri-apps/api/core";
import { memo, useEffect, useRef } from "react";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import css from "./File.module.css";
import FilePlaceholder from "./FilePlaceholder";
// import { listen } from "@tauri-apps/api/event";

type Props = {
  dir: string;
  file: DirWithFiles["files"][number];
};

const TILE_SIZE = 256; // px

const File = ({ dir: _, file }: Props) => {
  const { ref, inViewport } = useInViewport();

  const imgRef = useRef<HTMLImageElement>(null);
  const hasThumbnail = file.thumbs.length > 0;

  useEffect(() => {
    const thumbsWithGrid = file.thumbs.find((thumb) => thumb.grid);
    if (!thumbsWithGrid || !thumbsWithGrid.grid) {
      return;
    }

    const [columns, rows] = thumbsWithGrid.grid;
    const totalTiles = columns * rows;
    
    let intervalRef = null as any;
    let currentTile = 0;

    const updateTilePosition = () => {
      if (!imgRef.current) return;

      const row = Math.floor(currentTile / columns);
      const col = currentTile % columns;

      const xOffset = -col * TILE_SIZE;
      const yOffset = -row * TILE_SIZE;

      imgRef.current.style.objectPosition = `${xOffset}px ${yOffset}px`;

      currentTile = (currentTile + 1) % totalTiles;
    };

    intervalRef = setInterval(updateTilePosition, 150);

    return () => {
      if (intervalRef) {
        clearInterval(intervalRef);
      }
    };
  }, [file.thumbs]);

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

  const content = hasThumbnail ? (
    <img ref={imgRef} className={css.thumbnail} src={src} alt={file.name} />
  ) : (
    <FilePlaceholder file={file} />
  );

  return (
    <div ref={ref} className={css.fileWrapper}>
      {inViewport && content}
    </div>
  );
};

export default memo(File);
