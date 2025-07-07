import { useEffect } from "react";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";

export const useCursorSeekThumbnail = (
  thumbnail: DirWithFiles["files"][number]["thumbs"][number],
  ref: React.RefObject<HTMLImageElement>
) => {
  useEffect(() => {
    if (!ref.current) {
      console.log("no ref, error?");
      return;
    }

    const handleMouseMove = (event: MouseEvent) => {
      const { clientX, clientY } = event;
      const { left, top, width, height } = ref.current.getBoundingClientRect();

      const x = clientX - left;
      const y = clientY - top;

      console.log("Mouse position:", { x, y });

      // ref.current.style.objectPosition = `${x}px ${y}px`;
    };

    document.addEventListener("mousemove", handleMouseMove, { signal });
    return () => {
      abort();
    };
  }, [thumbnail]);
};
