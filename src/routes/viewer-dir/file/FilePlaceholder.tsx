import Icon from "@mdi/react";
import {
  mdiPlayCircleOutline,
  mdiHeadphones,
  mdiFileDocument,
  mdiImage
} from "@mdi/js";
import css from "./FilePlaceholder.module.css";
import { memo } from "react";
import { DirWithFiles } from "../../saved-folders/FilesInDirs.type";
import { getFileType } from "../../../util/util";
import { Text } from "@mantine/core";

type Props = {
  file: DirWithFiles["files"][number];
};

const FilePlaceholder = ({ file }: Props) => {
  const type = getFileType(file.name);
  return (
    <div className={css.filePlaceholder}>
      {type === "video" && <Icon path={mdiPlayCircleOutline} size={3} />}
      {type === "audio" && <Icon path={mdiHeadphones} size={3} />}
      {type === "image" && <Icon path={mdiImage} size={3} />}
      {type === "other" && <Icon path={mdiFileDocument} size={3} />}
      <Text c="dark" style={{ wordBreak: "break-word" }}>
        {file.name}
      </Text>
    </div>
  );
};

export default memo(FilePlaceholder);
