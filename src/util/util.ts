export const getFileType = (
  fileName: string
): "video" | "audio" | "image" | "other" => {
  const ext = fileName.split(".").pop()?.toLowerCase();
  if (!ext) return "other";

  if (["mp4", "mkv", "avi", "mov", "flv"].includes(ext)) return "video";
  if (["mp3", "wav", "flac", "aac"].includes(ext)) return "audio";
  if (["jpg", "jpeg", "png", "gif", "webp"].includes(ext)) return "image";

  return "other";
};
