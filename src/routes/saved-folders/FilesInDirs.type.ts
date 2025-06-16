export interface File {
  name: string;
  size: number;
}

export interface DirWithFiles {
  path: string;
  files: File[];
}

export interface FilesInDirs {
  dirs: Array<DirWithFiles>;
}
