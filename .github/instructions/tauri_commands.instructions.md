---
applyTo: "**/*.rs"
---

- Each command must be registered in `src-tauri\src\lib.rs` in `generate_handler`.
- State is handled with custom `src-tauri\src\state_manager.rs` module.

# Using `JsonState<T>`

- **Constraint**: Your state struct `T` must derive `Serialize`, `Deserialize`, and `Default`.
- **Load**: `let state = JsonState::<T>::load(path);`
- **Read**: `state.with(|s| { /* read-only access to s */ });`
- **Write**: `state.with_mut(|s| { /* mutable access to s */ });` (auto-saves to file)
- **Share**: `let handle_clone = state.clone();` (for use in other threads)
