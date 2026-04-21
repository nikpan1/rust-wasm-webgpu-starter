import type { WasmApp } from 'engine';
import styles from './ControlPanel.module.css';

interface ControlPanelProps {
  engineRef: React.RefObject<WasmApp | null>;
  isLoaded: boolean;
}

/**
 * Glassmorphism control panel with a "Speed Up" button.
 * Calls engine.increase_speed() via the ref — no React re-renders on click.
 */
export function ControlPanel({ engineRef, isLoaded }: ControlPanelProps) {
  function handleSpeedUp() {
    engineRef.current?.increase_speed();
  }

  return (
    <div className={styles.panel}>
      {/* Badge row */}
      <div className={styles.badges}>
        <span className={styles.badge}>Rust</span>
        <span className={styles.divider}>×</span>
        <span className={styles.badge}>WebAssembly</span>
        <span className={styles.divider}>×</span>
        <span className={styles.badge}>WebGPU</span>
      </div>

      {/* Speed Up button */}
      <button
        id="speed-up-btn"
        className={styles.btn}
        onClick={handleSpeedUp}
        disabled={!isLoaded}
        aria-label="Increase rotation speed"
      >
        <span className={styles.btnIcon}>⚡</span>
        <span className={styles.btnLabel}>Speed Up</span>
      </button>

      <p className={styles.hint}>
        Click to increase rotation speed.<br />
        Each click adds +0.01 rad/frame.
      </p>
    </div>
  );
}
