import { createBrowserRouter } from "react-router";
import Todos from "./todos/Todos";
import SavedDirs from "./saved-folders/SavedFolders";
import Config from "./config/Config";

export const router = createBrowserRouter([
  {
    path: "/",
    Component: Todos
  },
  {
    path: "/saved-dirs",
    Component: SavedDirs
  },
  {
    path: "/config",
    Component: Config
  }
]);
