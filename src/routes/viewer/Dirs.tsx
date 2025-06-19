import { FilesInDirs } from "../saved-folders/FilesInDirs.type";
import Dir from "./Dir";
import css from "./dirs.module.css";

type Props = {
  dirs: FilesInDirs["dirs"];
};

const Dirs = ({ dirs }: Props) => {
  return (
    <div className={css.dirs}>
      {dirs.map((dir) => (
        <Dir key={dir.path} dir={dir} />
      ))}
    </div>
  );
};

export default Dirs;
