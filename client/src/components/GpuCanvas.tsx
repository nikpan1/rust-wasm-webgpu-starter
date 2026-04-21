import React from 'react';
import styles from './GpuCanvas.module.css';

interface GpuCanvasProps {
  canvasRef: React.RefObject<HTMLCanvasElement | null>;
  isLoaded: boolean;
  error: string | null;
}

/**
 * Renders the WebGPU canvas inside a styled container.
 * The actual WebGPU initialisation happens in the useWasmEngine hook (parent).
 */
export function GpuCanvas({ canvasRef, isLoaded, error }: GpuCanvasProps) {
  return (
    <div className={styles.wrapper}>
      {/* Glow halo behind the canvas */}
      <div className={styles.halo} />

      <div className={styles.frame}>
        {/* The actual WebGPU render target */}
        <canvas
          id="gpu-canvas"
          ref={canvasRef}
          width={700}
          height={500}
          className={styles.canvas}
        />

        {/* Loading / error overlay */}
        {!isLoaded && !error && (
          <div className={styles.overlay}>
            <div className={styles.spinner} />
            <span className={styles.overlayText}>Initialising WebGPU…</span>
          </div>
        )}

        {error && (
          <div className={styles.overlay}>
            <div className={styles.errorIcon}>⚠</div>
            <span className={styles.errorText}>{error}</span>
            <p className={styles.errorHint}>
              Requires Chrome 113+ or Edge 113+ with WebGPU enabled.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
