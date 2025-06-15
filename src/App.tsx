// Import styles of packages that you've installed.
// All packages except `@mantine/hooks` require styles imports
import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";

import "./App.css";

import { MantineProvider } from "@mantine/core";
import { QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider } from "react-router";
import { queryClient } from "./queryClient";
import { router } from "./routes/router";
import { Notifications } from "@mantine/notifications";

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <MantineProvider>
        <RouterProvider router={router} />
        <Notifications />
      </MantineProvider>
    </QueryClientProvider>
  );
}

export default App;
