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
export function GpuCanvas({ canvasRef }: GpuCanvasProps) {
  return (
    <canvas
      id="gpu-canvas"
      ref={canvasRef}
      className={styles.canvas}
    />
  );
}
