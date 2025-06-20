import { LoadingOverlay, Stack, Title } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useParams } from "react-router";
import Layout from "../../components/Layout";
import { DirWithFiles } from "../saved-folders/FilesInDirs.type";
import File from "./file/File";
import css from "./ViewerDir.module.css";

// import { join } from "@tauri-apps/api/path";

const ViewerDir = () => {
  const params = useParams();

  const dirQuery = useQuery({
    queryKey: ["get_dir", params.dirPath],
    queryFn: () => invoke<DirWithFiles>("get_dir", { dir: params.dirPath }),
    enabled: !!params.dirPath // Only run if dirPath is defined
  });

  return (
    <Layout containerProps={{ size: "100%" }}>
      <Title order={2}>{params.dirPath}</Title>
      <Stack pos="relative">
        <LoadingOverlay visible={dirQuery.isLoading} />
        <div className={css.grid}>
          {dirQuery.data?.files.map((file) => (
            <File key={file.name} dir={dirQuery.data.path} file={file} />
          ))}
        </div>
      </Stack>
    </Layout>
  );
};

export default ViewerDir;
