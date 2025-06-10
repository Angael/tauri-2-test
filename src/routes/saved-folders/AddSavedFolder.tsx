import { Button, Group, TextInput } from "@mantine/core";
import { useState } from "react";

type Props = {
  onAddFolder: (folderName: string) => void;
  isPending: boolean;
};

const AddSavedFolder = ({ onAddFolder, isPending }: Props) => {
  const [folderPath, setFolderPath] = useState<string>("");

  const onSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const folderName = folderPath.trim();
    if (folderName) {
      onAddFolder(folderName);
      setFolderPath(""); // Clear input after adding
    }
  };

  return (
    <form onSubmit={onSubmit}>
      <Group>
        <TextInput
          placeholder="Folder name"
          value={folderPath}
          disabled={isPending}
          onChange={(event) => setFolderPath(event.currentTarget.value)}
        />
        <Button type="submit" loading={isPending}>
          Add
        </Button>
      </Group>
    </form>
  );
};

export default AddSavedFolder;
