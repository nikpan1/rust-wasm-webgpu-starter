import { useRef } from 'react';
import { GpuCanvas }    from './components/GpuCanvas';
import { ControlPanel } from './components/ControlPanel';
import { useWasmEngine } from './hooks/useWasmEngine';
import styles from './App.module.css';

/**
 * Root application component.
 *
 * Architecture:
 *  - canvasRef    → passed into <GpuCanvas> for DOM attachment
 *  - useWasmEngine → loads Wasm, inits WebGPU, drives rAF loop
 *  - engineRef    → passed to <ControlPanel> to call increase_speed()
 */
function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { engineRef, isLoaded, error } = useWasmEngine(canvasRef);

  return (
    <main className={styles.main}>
      {/* ── Header ─────────────────────────────────────── */}
      <header className={styles.header}>
        <div className={styles.pill}>
          <span className={styles.pillDot} />
          <span>WebGPU Live</span>
        </div>

        <h1 className={styles.title}>
          Rust&nbsp;
          <span className={styles.gradient}>×&nbsp;WebGPU</span>
          &nbsp;Showcase
        </h1>

        <p className={styles.subtitle}>
          A 3D rotating cube — engine written in Rust, compiled to WebAssembly,
          <br />rendered natively in the browser via WebGPU.
        </p>
      </header>

      {/* ── Canvas ─────────────────────────────────────── */}
      <GpuCanvas
        canvasRef={canvasRef}
        isLoaded={isLoaded}
        error={error}
      />

      {/* ── Controls ───────────────────────────────────── */}
      <ControlPanel
        engineRef={engineRef}
        isLoaded={isLoaded}
      />

      {/* ── Footer ─────────────────────────────────────── */}
      <footer className={styles.footer}>
        <span>Built with</span>
        <a
          href="https://www.rust-lang.org"
          target="_blank"
          rel="noopener noreferrer"
          className={styles.footerLink}
        >Rust</a>
        <span>·</span>
        <a
          href="https://wgpu.rs"
          target="_blank"
          rel="noopener noreferrer"
          className={styles.footerLink}
        >wgpu&nbsp;29</a>
        <span>·</span>
        <a
          href="https://rustwasm.github.io/wasm-pack/"
          target="_blank"
          rel="noopener noreferrer"
          className={styles.footerLink}
        >wasm-pack</a>
        <span>·</span>
        <a
          href="https://react.dev"
          target="_blank"
          rel="noopener noreferrer"
          className={styles.footerLink}
        >React&nbsp;19</a>
      </footer>
    </main>
  );
}

export default App;
