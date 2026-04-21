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
    <button
      id="speed-up-btn"
      className={styles.btn}
      onClick={handleSpeedUp}
      disabled={!isLoaded}
    >
      speed up
    </button>
  );
}
