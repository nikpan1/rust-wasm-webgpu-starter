# Rust x WebAssembly x WebGPU x React

A spinning cube. A concept project integrating Rust, WebGPU, and WebAssembly to offload 3D rendering from a React-based UI.

---

## Architecture

```mermaid
graph TD
    subgraph Browser["Browser (Chrome 113+)"]
        direction TB

        subgraph React["React 19 + TypeScript"]
            App["App.tsx"]
            Hook["useWasmEngine\n(rAF loop + lifecycle)"]
            Canvas["GpuCanvas"]
            Panel["ControlPanel\n(Speed Up button)"]
            App --> Hook
            App --> Canvas
            App --> Panel
        end

        subgraph Wasm["WebAssembly · wasm-bindgen"]
            WasmApp["WasmApp\ncreate · render · increase_speed"]
        end

        subgraph Engine["Rust Engine · wgpu 29"]
            State["State\n(Surface · Device · Queue)"]
            Geo["geometry.rs\n(24 verts · 36 indices)"]
            Math["math.rs\n(hand-rolled Mat4)"]
            Shader["shader.wgsl"]
            State --> Geo
            State --> Math
            State --> Shader
        end

        subgraph WebGPU["WebGPU"]
            GPU["GPU Pipeline\n(uniforms · depth · present)"]
        end

        Hook -- "await import('engine')" --> WasmApp
        Panel -- "engineRef.increase_speed()" --> WasmApp
        Hook -- "render() per frame" --> WasmApp
        WasmApp -- "FFI" --> State
        State -- "wgpu API" --> GPU
    end

    subgraph Build["Build Toolchain"]
        WasmPack["wasm-pack\n--target web"]
        Vite["Vite 8\nnpm run build"]
    end

    subgraph Server["Axum Server"]
        Serve["ServeDir · client/dist"]
        Headers["COOP / COEP headers"]
        Serve --> Headers
    end

    Server -- "HTTP :3000" --> Browser
    WasmPack --> Wasm
    Vite --> React

    classDef react   fill:#dbeafe,stroke:#93c5fd,color:#1e3a5f
    classDef wasm    fill:#fde8d8,stroke:#fb923c,color:#7c2d12
    classDef engine  fill:#fef3c7,stroke:#fbbf24,color:#78350f
    classDef webgpu  fill:#ede9fe,stroke:#a78bfa,color:#3b0764
    classDef build   fill:#dcfce7,stroke:#6ee7b7,color:#14532d
    classDef server  fill:#e2e8f0,stroke:#94a3b8,color:#0f172a

    class App,Hook,Canvas,Panel react
    class WasmApp wasm
    class State,Geo,Math,Shader engine
    class GPU webgpu
    class WasmPack,Vite build
    class Serve,Headers server
```

---

## Preview

![Preview](docs/demo.png)

---

## Prerequisites

```bash
# Rust + wasm32 target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
# wasm-pack
cargo install wasm-pack
# Node.js >= 18  →  https://nodejs.org
```

---

## Installation & Running
```bash
cd engine && wasm-pack build --target web --out-dir ../client/engine-pkg
```

```bash
cd client && npm install && npm run build
```
```bash
cargo run -p server
```

Open **http://localhost:3000**.

---

## Development

After making changes, only rebuild the affected layer:

```bash
# Rust engine changed
cd engine && wasm-pack build --target web --out-dir ../client/engine-pkg
cd ../client && npm run build && cd ..
```

```bash
# React / CSS / TypeScript changed
cd client && npm run build && cd ..
```

```bash
# Vite dev server (frontend-only, hot reload)
cd client && npm run dev   # http://localhost:5173
```

---

## Project Structure

```
webassembly-webgl-rust/
├── engine/src/         # Rust → Wasm (wgpu pipeline, geometry, math, WGSL shader)
├── server/src/         # Axum static file server
└── client/src/
    ├── hooks/          # useWasmEngine (Wasm init + rAF loop)
    └── components/     # GpuCanvas, ControlPanel
```
