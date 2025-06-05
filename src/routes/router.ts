import { createBrowserRouter } from "react-router";
import Todos from "./todos/Todos";
import SavedDirs from "./saved-dirs/SavedDirs";

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
