import { convertFileSrc } from "@tauri-apps/api/core";
import css from "./File.module.css";
import {
  DirWithFiles,
  ThumbnailType
} from "../../saved-folders/FilesInDirs.type";
import { memo, use, useCallback, useRef } from "react";
import { path } from "@tauri-apps/api";

const cacheDirPromise = path.appCacheDir();

const getTilePos = (thumb: ThumbnailType, i: number): [number, number] => {
  const { grid, res } = thumb;
  if (!grid) {
    return [0, 0];
  }

  const [columns, _rows] = grid;
  const [tileWidth, tileHeight] = res;

  const row = Math.floor(i / columns);
  const col = i % columns;

  const x = -col * tileWidth;
  const y = -row * tileHeight;

  return [x, y];
};

const getTileIndex = (percentage: number, tileCount: number) => {
  const index = Math.floor(percentage * tileCount);
  return index < 0 ? 0 : index;
};

type Props = {
  file: DirWithFiles["files"][number];
};

const Thumbnail = ({ file }: Props) => {
  // Is the thumbnail grid animating on it's own on interval?
  const playing = useRef(true);
  const thumbWithGrid = file.thumbs.find((thumb) => thumb.grid);
  const cacheDir = use(cacheDirPromise);

  const imgRef = useCallback(
    (node: HTMLImageElement | null) => {
      if (!node || !thumbWithGrid?.grid) return;
      const abortControler = new AbortController();
      const tileCount = thumbWithGrid.grid![0] * thumbWithGrid.grid![1];

      let index = 1;
      const handleMouseMove = (event: MouseEvent) => {
        playing.current = false; // Stop the interval when mouse moves

        const { left, width } = node.getBoundingClientRect();
        const percentageX = (event.clientX - left) / width;

        index = getTileIndex(percentageX, tileCount);
        const [x, y] = getTilePos(thumbWithGrid, index);

        console.log("a", { percentageX, index, x, y });

        node.style.objectPosition = `${x}px ${y}px`;
      };

      const interval = setInterval(() => {
        if (!playing.current) return;
        const [x, y] = getTilePos(thumbWithGrid, index);
        node.style.objectPosition = `${x}px ${y}px`;

        index = (index + 1) % tileCount;
      }, 500);

      node.addEventListener("mousemove", handleMouseMove, {
        signal: abortControler.signal
      });
      node.addEventListener(
        "mouseleave",
        () => {
          playing.current = true;
        },
        { signal: abortControler.signal }
      );
      return () => {
        abortControler.abort();
        clearInterval(interval);
      };
    },
    [thumbWithGrid]
  );
  // useCursorSeekThumbnail(file.thumbs[0], imgRef);

  const src = convertFileSrc(`${cacheDir}\\files\\${file.id}\\thumbnail.avif`);

  return (
    <img ref={imgRef} className={css.thumbnail} src={src} alt={file.name} />
  );
};

export default memo(Thumbnail);
