import { createBrowserRouter } from "react-router";
import Todos from "./todos/Todos";
import SavedDirs from "./saved-folders/SavedFolders";

export const router = createBrowserRouter([
  {
    path: "/",
    Component: Todos
  },
  {
    path: "/saved-dirs",
    Component: SavedDirs
  }
]);
