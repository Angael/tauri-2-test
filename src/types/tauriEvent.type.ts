export type GenerateThumbEvent = {
  dir: string;
  id: string;
};

export type DirScanProgressEvent = {
  dir: string;
  i: number;
  total: number;
};
