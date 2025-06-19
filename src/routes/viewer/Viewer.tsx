import { Title } from "@mantine/core";
import Layout from "../../components/Layout";
import { useDirs } from "../saved-folders/useDirs";
import Dirs from "./Dirs";

const Viewer = () => {
  const dirsQuery = useDirs();

  return (
    <Layout containerProps={{ size: "xl" }}>
      <Title order={1}>File Viewer</Title>
      {dirsQuery.isLoading && "Loading..."}
      {dirsQuery.isError && "Error loading directories."}
      {dirsQuery.data && <Dirs dirs={dirsQuery.data.dirs} />}
    </Layout>
  );
};

export default Viewer;
