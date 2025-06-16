# Tauri 2 + React 19 + TypeScript

A modern desktop application built with Tauri 2, React 19, and TypeScript. Features a beautiful UI with Mantine components, efficient state management with React Query, and a powerful Rust backend.

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (Latest stable)
- [Bun](https://bun.sh/) (Fast JavaScript runtime and package manager)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd tauri-2-test

# Install JavaScript dependencies
bun install

# Update Rust dependencies
cd src-tauri
cargo update
cd ..
```

### Development

```bash
# Start the development server (frontend + backend)
bun tauri dev

# Or start only the frontend (for UI development)
bun dev
```

## 📦 Available Commands

### Development

```bash
bun dev          # Start Vite dev server (frontend only)
bun tauri dev    # Start Tauri app in development mode
bun preview      # Preview production build locally
```

### Building

```bash
bun run build       # Build frontend for production
bun tauri build # Build complete Tauri application
```

### Code Quality

```bash
bun fix         # Format and lint Rust code
```

### VS Code Tasks

- **ui:dev** - Start frontend development server (background)
- **ui:build** - Build frontend for production

Use `Ctrl+Shift+P` → "Tasks: Run Task" to access these.

## 🛠️ Technology Stack

### Frontend

- **React 19** - Latest React with improved performance
- **TypeScript** - Type-safe JavaScript
- **Vite 6** - Lightning-fast build tool
- **Mantine 8** - Modern UI component library
- **React Query** - Powerful data fetching and caching
- **React Router 7** - Client-side routing
- **Recharts** - Beautiful chart components

### Backend

- **Rust** - Memory-safe systems programming
- **Tauri 2** - Secure desktop app framework
- **Serde** - Serialization framework

### Tools

- **Bun** - Fast package manager and runtime
- **PostCSS** - CSS processing with Mantine preset

## 📁 Project Structure

```
├── src/                    # React frontend
│   ├── components/         # Reusable UI components
│   ├── routes/            # Application pages and routing
│   ├── assets/            # Static assets (images, icons)
│   └── util/              # Helper functions
├── src-tauri/             # Rust backend
│   ├── src/               # Rust source code
│   ├── capabilities/      # Tauri security capabilities
│   └── icons/             # Application icons
├── public/                # Static public assets
└── .github/               # GitHub workflows and instructions
```

## 🔧 Configuration

### Key Configuration Files

- `src-tauri/tauri.conf.json` - Tauri app configuration
- `src-tauri/Cargo.toml` - Rust dependencies
- `vite.config.ts` - Vite build configuration
- `package.json` - Node.js dependencies and scripts
- `tsconfig.json` - TypeScript configuration

### Environment Setup

- **Development**: Runs on `http://localhost:1420`
- **Build Output**: `dist/` (frontend), `src-tauri/target/` (backend)

## 🚦 Development Workflow

1. **Setup**: Follow installation steps above
2. **Development**: Run `bun run tauri dev`
3. **Frontend Changes**: Auto-reload enabled
4. **Backend Changes**: Restart dev command
5. **Testing**: Build with `bun run tauri build`

## 🐛 Troubleshooting

### Common Issues

**Port 1420 already in use:**

```bash
# Kill process using the port
netstat -ano | findstr :1420
taskkill /PID <PID> /F
```

**Rust compilation errors:**

```bash
cd src-tauri
cargo clean
cargo check
```

**Dependencies out of sync:**

```bash
bun install
cd src-tauri && cargo update
```

**Build failures:**

```bash
# Clear all caches
bun run tauri clean
rm -rf node_modules dist
bun install
```

## 📚 Learn More

- [Tauri Documentation](https://tauri.app/)
- [React 19 Documentation](https://react.dev/)
- [Mantine Components](https://mantine.dev/)
- [React Query Guide](https://tanstack.com/query/latest)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
