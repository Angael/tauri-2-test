import { createBrowserRouter } from "react-router";
import SavedDirs from "./saved-folders/SavedFolders";
import Config from "./config/Config";

export const router = createBrowserRouter([
  {
    path: "/",
    Component: SavedDirs,
    index: true
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
