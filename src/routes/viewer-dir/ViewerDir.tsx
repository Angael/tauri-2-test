import { Group, LoadingOverlay, Stack, Title } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useRef } from "react";
import { useParams } from "react-router";
import StackContainer from "../../components/StackContainer";
import { DirWithFiles } from "../saved-folders/FilesInDirs.type";
import FilePreview from "./file-preview/FilePreview";
import File from "./file/File";
import css from "./ViewerDir.module.css";

const ViewerDir = () => {
  const params = useParams();
  const previewRef = useRef<HTMLDivElement>(null);

  const dirQuery = useQuery({
    queryKey: ["get_dir", params.dirPath],
    queryFn: () => invoke<DirWithFiles>("get_dir", { dir: params.dirPath }),
    enabled: !!params.dirPath // Only run if dirPath is defined
  });

  return (
    <StackContainer size="100%">
      <Group pos="relative" align="stretch" wrap="nowrap">
        <LoadingOverlay visible={dirQuery.isLoading} />
        <Stack flex={1}>
          <Title order={1}>{params.dirPath}</Title>
          <div className={css.grid}>
            {dirQuery.data?.files.map((file) => (
              <File key={file.name} dir={dirQuery.data.path} file={file} />
            ))}
          </div>
        </Stack>

        <FilePreview ref={previewRef} dirPath={params.dirPath!} />
      </Group>
    </StackContainer>
  );
};

export default ViewerDir;
