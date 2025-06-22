export interface File {
  id: string;
  name: string;
  size: number;

  video_stats?: {
    /** Duration in seconds */
    dur: number;

    /** Resolution of the video (width, height) in px */
    res: [number, number];

    /** Bitrate in kbps */
    br: number;
  };
}

export interface DirWithFiles {
  path: string;
  files: File[];
}

export interface FilesInDirs {
  dirs: Array<DirWithFiles>;
}
