export interface Thumbnail {
  res: [number, number];
  grid?: [number, number];
}

export interface File {
  id: string;
  name: string;
  size: number;

  thumbs: Thumbnail[];
}

export interface DirWithFiles {
  path: string;
  files: File[];
}

export interface FilesInDirs {
  dirs: Array<DirWithFiles>;
}
