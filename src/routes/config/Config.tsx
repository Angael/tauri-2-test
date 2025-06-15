import { Button, TextInput } from "@mantine/core";
import Layout from "../../components/Layout";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

type Props = {};

const Config = (props: Props) => {
  const configQuery = useQuery({
    queryKey: ["get_config"],
    queryFn: async () => {
      return invoke("get_config");
    }
  });

  return (
    <Layout>
      <pre>{JSON.stringify(configQuery, null, 2)}</pre>
      <TextInput
        label="Path to ffmpeg"
        placeholder="I:\FFmpeg\bin\ffmpeg.exe"
      />
      <TextInput
        label="Path to ffprobe"
        placeholder="I:\FFmpeg\bin\ffprobe.exe"
      />
      <Button
        onClick={async () => {
          await invoke("set_config", {
            ffmpegPath: "I:\\FFmpeg\\bin\\ffmpeg.exe",
            ffprobePath: "I:\\FFmpeg\\bin\\ffprobe.exe"
          });
          configQuery.refetch();
        }}
      >
        Save Config
      </Button>
    </Layout>
  );
};

export default Config;
