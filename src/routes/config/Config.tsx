import { Button, TextInput } from "@mantine/core";
import { useMutation, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import Layout from "../../components/Layout";

interface ConfigData {
  ffmpeg_path: string;
  ffprobe_path: string;
}

const Config = () => {
  const configQuery = useQuery({
    queryKey: ["get_config"],
    queryFn: async () => {
      return invoke<ConfigData>("get_config");
    }
  });

  const configMutation = useMutation({
    mutationFn: async (config: { ffmpegPath: string; ffprobePath: string }) => {
      return invoke("set_config", {
        ffmpegPath: config.ffmpegPath,
        ffprobePath: config.ffprobePath
      });
    },
    onSuccess: () => {
      configQuery.refetch();
    }
  });

  const [ffmpegPath, setFfmpegPath] = useState("");
  const [ffprobePath, setFfprobePath] = useState("");

  useEffect(() => {
    if (configQuery.data) {
      setFfmpegPath(configQuery.data.ffmpeg_path || "");
      setFfprobePath(configQuery.data.ffprobe_path || "");
    }
  }, [configQuery.data]);

  const handleFfmpegPathChange = (value: string) => {
    setFfmpegPath(value);
  };

  const handleFfprobePathChange = (value: string) => {
    setFfprobePath(value);
  };

  const handleSaveConfig = () => {
    configMutation.mutate({
      ffmpegPath,
      ffprobePath
    });
  };

  const hasUnsavedChanges =
    ffmpegPath !== configQuery.data?.ffmpeg_path ||
    ffprobePath !== configQuery.data?.ffprobe_path;

  return (
    <Layout>
      <TextInput
        label="Path to ffmpeg"
        placeholder="I:\FFmpeg\bin\ffmpeg.exe"
        value={ffmpegPath}
        onChange={(event) => handleFfmpegPathChange(event.currentTarget.value)}
      />
      <TextInput
        label="Path to ffprobe"
        placeholder="I:\FFmpeg\bin\ffprobe.exe"
        value={ffprobePath}
        onChange={(event) => handleFfprobePathChange(event.currentTarget.value)}
      />
      <Button
        onClick={handleSaveConfig}
        loading={configMutation.isPending}
        disabled={configMutation.isPending || !hasUnsavedChanges}
      >
        Save Config
      </Button>
    </Layout>
  );
};

export default Config;
