import { createBrowserRouter } from "react-router";
import Config from "./config/Config";
import SavedDirs from "./saved-folders/SavedFolders";
import ViewerDir from "./viewer-dir/ViewerDir";
import Layout from "../components/Layout";
import Debug from "./debug/Debug";

export const router = createBrowserRouter([
  {
    path: "/",
    Component: SavedDirs,
    index: true
  },
  {
    path: "/viewer-dir/:dirPath",
    Component: ViewerDir
  },
  {
    path: "/config",
    Component: Config
  },
  {
    path: "/debug",
    Component: Debug
  },
  // catch all
  {
    path: "*",
    Component: Layout
  }
]);
