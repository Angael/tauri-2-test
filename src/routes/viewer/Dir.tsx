import { Group, Stack, Text } from "@mantine/core";
import { FilesInDirs } from "../saved-folders/FilesInDirs.type";
import css from "./Dir.module.css";
import { useNavigate } from "react-router";

type Props = {
  dir: FilesInDirs["dirs"][number];
};

const Dir = ({ dir }: Props) => {
  const dirName = dir.path.split(/[\\/]/).pop() || "Directory";
  const navigate = useNavigate();

  const files = dir.files.length;

  const onClick = () => {
    navigate(`/viewer-dir/${encodeURIComponent(dir.path)}`);
  };

  return (
    <div className={css.btn} onClick={onClick}>
      <Group>
        <Stack gap={0}>
          <Text size="xs" c="gray.6">
            {dir.path}
          </Text>
          <Text size="lg">{dirName}</Text>
        </Stack>

        <Stack ml="auto">
          <Text>
            {files} {files === 1 ? "file" : "files"}
          </Text>
        </Stack>
      </Group>
    </div>
  );
};

export default Dir;
