import { createBrowserRouter } from "react-router";
import Config from "./config/Config";
import SavedDirs from "./saved-folders/SavedFolders";
import ViewerDir from "./viewer-dir/ViewerDir";
import Layout from "../components/Layout";
import Debug from "./debug/Debug";
import { Outlet } from "react-router";

export const router = createBrowserRouter([
  {
    path: "/",
    element: (
      <Layout>
        <Outlet />
      </Layout>
    ),
    children: [
      {
        index: true,
        element: <SavedDirs />
      },
      {
        path: "viewer-dir/:dirPath",
        element: <ViewerDir />
      },
      {
        path: "config",
        element: <Config />
      },
      {
        path: "debug",
        element: <Debug />
      },
      // catch all
      {
        path: "*",
        element: <SavedDirs />
      }
    ]
  }
]);
