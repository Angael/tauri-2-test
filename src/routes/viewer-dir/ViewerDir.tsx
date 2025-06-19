import { Group, LoadingOverlay, Stack, Title } from "@mantine/core";
import { useLocation, useParams } from "react-router";
import Layout from "../../components/Layout";
import { useQuery } from "@tanstack/react-query";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { DirWithFiles } from "../saved-folders/FilesInDirs.type";

const ViewerDir = () => {
  const location = useLocation();
  const params = useParams();

  const dirQuery = useQuery({
    queryKey: ["get_dir", params.dirPath],
    queryFn: () => invoke<DirWithFiles>("get_dir", { dir: params.dirPath }),
    enabled: !!params.dirPath // Only run if dirPath is defined
  });

  console.log({
    location,
    params,
    dirQuery: {
      isLoading: dirQuery.isLoading,
      isError: dirQuery.isError,
      data: dirQuery.data
    }
  });

  return (
    <Layout containerProps={{ size: "xl" }}>
      <Title order={2}>{params.dirPath}</Title>
      <Stack pos="relative">
        <LoadingOverlay visible={dirQuery.isLoading} />
        <Group wrap="wrap">
          {dirQuery.data?.files.map((file) => (
            <img
              src={convertFileSrc(dirQuery.data?.path + "\\" + file.name)}
              alt={file.name}
              key={file.path}
              style={{
                width: "100px",
                height: "100px",
                objectFit: "cover",
                margin: "5px"
              }}
            />
          ))}
        </Group>
        <pre>
          {dirQuery.isLoading && "Loading..."}
          {dirQuery.isError && "Error loading directory."}
          {dirQuery.data ? (
            JSON.stringify(dirQuery.data, null, 2)
          ) : (
            <span>No data available</span>
          )}
        </pre>
      </Stack>
    </Layout>
  );
};

export default ViewerDir;
