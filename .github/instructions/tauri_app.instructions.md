---
applyTo: "**"
---

# Tauri 2 App Instructions

This is a Tauri 2 desktop application built with React, TypeScript, and Rust. The frontend uses Vite for development and building, with Mantine UI components and React Query for state management.

## Project Structure

```
├── src/                    # React frontend source code
│   ├── components/         # React components
│   ├── routes/            # Application routes and pages
│   ├── assets/            # Static assets
│   └── util/              # Utility functions
├── src-tauri/             # Rust backend source code
│   ├── src/               # Rust source files
│   ├── capabilities/      # Tauri capabilities configuration
│   ├── icons/             # App icons
│   └── target/            # Rust build artifacts (auto-generated)
├── public/                # Static public assets
└── dist/                  # Frontend build output (auto-generated)
```

## Technology Stack

### Frontend

- **React 19** - UI framework
- **TypeScript** - Type-safe JavaScript
- **Vite 6** - Build tool and dev server
- **Mantine 8** - UI component library
- **React Query** - Server state management
- **React Router 7** - Client-side routing
- **Recharts** - Chart components, used through Mantine wrappers for visuals

### Backend

- **Rust** - Systems programming language
- **Tauri 2** - Desktop app framework
- **Serde** - Serialization/deserialization

### Package Manager

- **Bun** - Fast JavaScript runtime and package manager

## Development Setup

### Prerequisites

- Node.js (Latest LTS)
- Bun package manager
- Rust (Latest stable)
- Tauri CLI

### Installation

1. Install Rust: https://rustup.rs/
2. Install Bun: https://bun.sh/
3. Install Tauri CLI: `cargo install @tauri-apps/cli@next`
4. Install dependencies: `bun install`

### Development Commands

#### Frontend Development

```bash
# Start frontend dev server only
bun run dev

# Build frontend
bun run build

# Preview production build
bun run preview
```

#### Tauri Development

```bash
# Start Tauri dev mode (launches app with hot reload)
bun run tauri dev

# Build Tauri app for production
bun run tauri build

# Build frontend and backend together
bun run tauri build
```

#### Code Quality

```bash
# Fix Rust formatting and linting
bun run fix
```

### Available VS Code Tasks

- `ui:dev` - Start frontend development server (background task)
- `ui:build` - Build frontend for production

## Configuration Files

### Frontend Configuration

- `vite.config.ts` - Vite configuration
- `tsconfig.json` - TypeScript configuration
- `postcss.config.cjs` - PostCSS configuration
- `package.json` - Node.js dependencies and scripts

### Tauri Configuration

- `src-tauri/tauri.conf.json` - Main Tauri configuration
- `src-tauri/Cargo.toml` - Rust dependencies
- `src-tauri/capabilities/default.json` - App capabilities and permissions

## Key Features

### Frontend Features

- Modern React 19 with TypeScript
- Mantine UI components for consistent design
- React Query for efficient data fetching
- React Router for navigation
- Chart visualization with Recharts
- CSS Modules for component styling

### Backend Features

- Rust-based backend with Tauri 2
- File system operations
- Configuration management
- State management across app lifecycle

## Development Workflow

1. **Start Development**: Run `bun run tauri dev` to start both frontend and backend
2. **Frontend Changes**: Hot reload is enabled for React components
3. **Backend Changes**: Rust code changes require restart of dev command
4. **Building**: Use `bun run tauri build` for production builds

## File Organization

### Frontend (`src/`)

- `main.tsx` - React app entry point
- `App.tsx` - Main app component
- `queryClient.ts` - React Query configuration
- `components/` - Reusable UI components
- `routes/` - Page components and routing logic
- `util/` - Helper functions and utilities

### Backend (`src-tauri/src/`)

- `main.rs` - Tauri app entry point
- `lib.rs` - Library exports
- `app_state.rs` - Application state management
- `state_manager.rs` - State persistence
- Feature modules organized in subdirectories

## Best Practices

### Frontend

- Use TypeScript for all new files
- Follow React hooks patterns
- Use Mantine components when possible
- Implement proper error handling with notifications
- Use React Query for server state

### Backend

- Follow Rust conventions and use `cargo fmt`
- Implement proper error handling
- Use Tauri's invoke system for frontend-backend communication
- Organize code into logical modules

### General

- Keep dependencies up to date
- Use meaningful commit messages
- Test changes in development mode before building
- Follow the established project structure

## Common Commands

```bash
# Full development workflow
bun install                 # Install dependencies
bun run tauri dev          # Start development
bun run tauri build        # Build for production

# Debugging
bun run tauri dev --debug  # Start with debug info
cargo check                # Check Rust code (in src-tauri/)
```

## Troubleshooting

### Build Issues

- Ensure all dependencies are installed: `bun install`
- Check Rust toolchain: `rustc --version`
- Clear build cache: `cargo clean` (in src-tauri/)

### Development Issues

- Check if ports 1420 is available
- Restart dev server if hot reload stops working
- Check console for TypeScript errors

### Performance

- Use React DevTools for frontend debugging
- Use Rust's built-in profiling tools for backend optimization
- Monitor bundle size with Vite's build analysis
