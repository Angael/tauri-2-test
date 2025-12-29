export function splitPathAndFilename(path: string): [string, string] {
  const parts = path.split(/[/\\]/);
  const folderName = parts.pop() || "";
  const dirPath = parts.join("/");
  return [dirPath, folderName];
}
