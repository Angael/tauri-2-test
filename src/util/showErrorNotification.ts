import { showNotification } from "@mantine/notifications";

export const showErrorNotification = (title: string, message: string) =>
  showNotification({
    color: "red",
    title,
    message
  });
