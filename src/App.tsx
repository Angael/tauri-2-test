// Import styles of packages that you've installed.
// All packages except `@mantine/hooks` require styles imports
import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";

import "./App.css";

import { createTheme, MantineProvider } from "@mantine/core";
import { QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider } from "react-router";
import { queryClient } from "./queryClient";
import { router } from "./routes/router";
import { Notifications } from "@mantine/notifications";

const theme = createTheme({
  defaultRadius: "lg",
  primaryColor: "grape"
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <MantineProvider theme={theme}>
        <RouterProvider router={router} />
        <Notifications />
      </MantineProvider>
    </QueryClientProvider>
  );
}

export default App;
