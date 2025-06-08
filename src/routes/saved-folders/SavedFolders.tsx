import { Button, Text, TextInput } from "@mantine/core";
import { useListState } from "@mantine/hooks";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";
import Layout from "../../components/Layout";

interface SavedFolder {
  path: string;
}

const SavedDirs = () => {
  const [savedDirsState, saveDirsHandlers] = useListState<SavedFolder>([]);

  const savedDirs = useQuery({
    queryKey: ["get_saved_folders2"],
    queryFn: async () => {
      console.log(1);
      const _folders = await invoke<SavedFolder[]>("get_saved_folders");
      console.log("2", _folders);
      return _folders;
    }
    // staleTime: Infinity
  });

  console.log(savedDirs);

  const isSaved = savedDirsState.every((dir) =>
    savedDirs.data?.some((savedDir) => savedDir.path === dir.path)
  );

  useEffect(() => {
    if (savedDirs.status === "success" && savedDirs.data) {
      saveDirsHandlers.setState(savedDirs.data);
    }
  }, [savedDirs.status, savedDirs.data]);

  // const saveDirs = useMutation({
  //   mutationKey: ["save_dirs"],
  //   mutationFn: (dirName: string) =>
  //     invoke("save_dirs", { dir_names: dirName }),
  //   onSuccess: () => {
  //     savedDirs.refetch();
  //   }
  // });

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    // if (event.key === "Enter") {
    //   const dirName = event.currentTarget.value.trim();
    //   if (dirName) {
    //     saveDirs.mutate(dirName);
    //     event.currentTarget.value = ""; // Clear input after saving
    //   }
    // }
  };

  return (
    <Layout>
      <Text c={isSaved ? "green" : "red"} size="xl" mb="md">
        {isSaved ? "All directories are saved" : "Some directories are unsaved"}
      </Text>
      {savedDirsState.map((dir, index) => (
        <div key={index}>
          <TextInput
            label="Directory Name"
            placeholder="Enter directory name"
            value={dir.path}
            onChange={(e) =>
              saveDirsHandlers.setItem(index, { path: e.currentTarget.value })
            }
          />
        </div>
      ))}
      <Button
        disabled={isSaved}
        onClick={() => {
          invoke("save_folders", { saved_folders: savedDirsState })
            .then(() => {
              savedDirs.refetch();
            })
            .catch((error) => {
              console.error("Failed to save directories:", error);
            });
        }}
      >
        Save directories
      </Button>
      Output:
      <pre>
        <code>{JSON.stringify(savedDirs.data, null, 2)}</code>
      </pre>
    </Layout>
  );
};

export default SavedDirs;
