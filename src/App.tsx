import { useState, useEffect, FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core"; // Updated import for Tauri v2

// Define the Todo interface for TypeScript
interface Todo {
  id: number;
  text: string;
  completed: boolean;
}

function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [inputText, setInputText] = useState("");

  // Function to fetch todos from the backend
  const fetchTodos = async () => {
    try {
      const fetchedTodos: Todo[] = await invoke("get_todos");
      setTodos(fetchedTodos);
    } catch (error) {
      console.error("Failed to fetch todos:", error);
      // Handle error (e.g., show a message to the user)
    }
  };

  // Load todos when the component mounts
  useEffect(() => {
    fetchTodos();
  }, []);

  const handleAddTodo = async (e: FormEvent) => {
    e.preventDefault();
    if (!inputText.trim()) return;
    try {
      // The add_todo command now returns the created todo
      const newTodo: Todo = await invoke("add_todo", { text: inputText });
      setTodos((prevTodos) => [...prevTodos, newTodo]); // Optimistically update or re-fetch
      setInputText("");
      // fetchTodos(); // Or update state directly if add_todo returns the new item
    } catch (error) {
      console.error("Failed to add todo:", error);
    }
  };

  const handleToggleTodo = async (id: number) => {
    try {
      await invoke("toggle_todo", { id });
      setTodos((prevTodos) =>
        prevTodos.map((todo) =>
          todo.id === id ? { ...todo, completed: !todo.completed } : todo
        )
      );
      // fetchTodos(); // Or update state directly
    } catch (error) {
      console.error("Failed to toggle todo:", error);
    }
  };

  const handleRemoveTodo = async (id: number) => {
    try {
      await invoke("remove_todo", { id });
      setTodos((prevTodos) => prevTodos.filter((todo) => todo.id !== id));
      // fetchTodos(); // Or update state directly
    } catch (error) {
      console.error("Failed to remove todo:", error);
    }
  };

  return (
    <div style={{ padding: "20px", fontFamily: "sans-serif" }}>
      <h1>Tauri Todo App</h1>
      <form onSubmit={handleAddTodo} style={{ marginBottom: "20px" }}>
        <input
          type="text"
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          placeholder="Add a new todo"
          style={{ marginRight: "10px", padding: "8px" }}
        />
        <button type="submit" style={{ padding: "8px 12px" }}>
          Add Todo
        </button>
      </form>
      <ul style={{ listStyle: "none", padding: 0 }}>
        {todos.map((todo) => (
          <li
            key={todo.id}
            style={{
              display: "flex",
              alignItems: "center",
              marginBottom: "10px",
              padding: "10px",
              border: "1px solid #eee",
              borderRadius: "4px"
            }}
          >
            <input
              type="checkbox"
              checked={todo.completed}
              onChange={() => handleToggleTodo(todo.id)}
              style={{ marginRight: "10px" }}
            />
            <span
              style={{
                textDecoration: todo.completed ? "line-through" : "none",
                flexGrow: 1,
                cursor: "pointer"
              }}
              onClick={() => handleToggleTodo(todo.id)}
            >
              {todo.text} (ID: {todo.id})
            </span>
            <button
              onClick={() => handleRemoveTodo(todo.id)}
              style={{
                marginLeft: "10px",
                padding: "5px 10px",
                background: "#ffdddd",
                border: "none",
                borderRadius: "4px",
                cursor: "pointer"
              }}
            >
              Remove
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
