import { path } from "@tauri-apps/api";
import Layout from "../../components/Layout";
import { use } from "react";
import { Code, Stack, Title } from "@mantine/core";
import { useDirs } from "../saved-folders/useDirs";

const cacheDirPromise = path.appCacheDir();
const dataDirPromise = path.appDataDir();

const Debug = () => {
  const dirsQuery = useDirs();
  const cacheDir = use(cacheDirPromise);
  const dataDir = use(dataDirPromise);

  return (
    <Layout>
      <Stack>
        <Title order={1}>Debug</Title>
        <p>Cache directory:</p>
        <Code>{cacheDir}</Code>

        <p>Data directory:</p>
        <Code>{dataDir}</Code>

        <Title order={2}>State</Title>
        <Code block>{JSON.stringify(dirsQuery.data, null, 2)}</Code>
      </Stack>
    </Layout>
  );
};

export default Debug;
