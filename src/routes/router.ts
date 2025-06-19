import { createBrowserRouter } from "react-router";
import SavedDirs from "./saved-folders/SavedFolders";
import Config from "./config/Config";
import Viewer from "./viewer/Viewer";
import ViewerDir from "./viewer-dir/ViewerDir";

export const router = createBrowserRouter([
  {
    path: "/",
    Component: Viewer,
    index: true
  },
  {
    path: "/viewer-dir/:dirPath",
    Component: ViewerDir
  },
  {
    path: "/saved-dirs",
    Component: SavedDirs,
    index: true
  },
  {
    path: "/config",
    Component: Config
  }
]);
