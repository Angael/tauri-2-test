import { Code, Stack, Title } from "@mantine/core";
import { path } from "@tauri-apps/api";
import { use } from "react";
import { useDirs } from "../saved-folders/useDirs";
import StackContainer from "../../components/StackContainer";

const cacheDirPromise = path.appCacheDir();
const dataDirPromise = path.appDataDir();

const Debug = () => {
  const dirsQuery = useDirs();
  const cacheDir = use(cacheDirPromise);
  const dataDir = use(dataDirPromise);

  return (
    <StackContainer>
      <Stack>
        <Title order={1}>Debug</Title>
        <p>Cache directory:</p>
        <Code>{cacheDir}</Code>

        <p>Data directory:</p>
        <Code>{dataDir}</Code>

        <Title order={2}>State</Title>
        <Code block>{JSON.stringify(dirsQuery.data, null, 2)}</Code>
      </Stack>
    </StackContainer>
  );
};

export default Debug;
