import { Group, LoadingOverlay, Title } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useParams } from "react-router";
import { useStore } from "zustand";
import StackContainer from "../../components/StackContainer";
import { $currentPreview } from "../../stores/current_preview";
import { DirWithFiles } from "../saved-folders/FilesInDirs.type";
import FilePreview from "./file-preview/FilePreview";
import File from "./file/File";
import css from "./ViewerDir.module.css";

const ViewerDir = () => {
  const params = useParams();
  const previewFile = useStore($currentPreview, (s) => s.file);
  const setPreviewFile = useStore($currentPreview, (s) => s.set);

  const dirQuery = useQuery({
    queryKey: ["get_dir", params.dirPath],
    queryFn: () => invoke<DirWithFiles>("get_dir", { dir: params.dirPath }),
    enabled: !!params.dirPath // Only run if dirPath is defined
  });

  return (
    <StackContainer size="100%">
      <Title order={1}>
        {params.dirPath} {previewFile?.name}
      </Title>
      <Group pos="relative" align="stretch">
        <LoadingOverlay visible={dirQuery.isLoading} />
        <div className={css.grid}>
          {dirQuery.data?.files.map((file) => (
            <File key={file.name} dir={dirQuery.data.path} file={file} />
          ))}
        </div>
        {previewFile && (
          <FilePreview
            file={previewFile}
            onClose={() => setPreviewFile(null)}
          />
        )}
      </Group>
    </StackContainer>
  );
};

export default ViewerDir;
