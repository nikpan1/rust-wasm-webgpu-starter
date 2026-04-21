import { useEffect, useRef, useState } from 'react';
import type { WasmApp } from 'engine';

export interface WasmEngineState {
  engineRef: React.RefObject<WasmApp | null>;
  isLoaded: boolean;
  error: string | null;
}

/**
 * Loads the Wasm engine, initialises WebGPU on the given canvas, and drives
 * the requestAnimationFrame render loop.
 *
 * The hook is designed to run only once on mount and clean up on unmount.
 */
export function useWasmEngine(
  canvasRef: React.RefObject<HTMLCanvasElement | null>,
): WasmEngineState {
  const engineRef = useRef<WasmApp | null>(null);
  const [isLoaded, setIsLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let rafId = 0;
    let disposed = false;

    async function bootstrap() {
      const canvas = canvasRef.current;
      if (!canvas) {
        setError('Canvas element not found.');
        return;
      }

      try {
        // ── 1. Load the Wasm module ────────────────────────────────────
        // Dynamic import keeps the chunk out of the main bundle.
        const wasmModule = await import('engine');
        // init() fetches + compiles engine_bg.wasm
        await wasmModule.default();

        if (disposed) return;

        // ── 2. Initialise WebGPU via our Rust engine ───────────────────
        const app = await wasmModule.WasmApp.create(canvas);

        if (disposed) {
          app.free();
          return;
        }

        engineRef.current = app;
        setIsLoaded(true);

        // ── 3. Drive the render loop ────────────────────────────────────
        function loop() {
          if (disposed || !engineRef.current) return;
          engineRef.current.render();
          rafId = requestAnimationFrame(loop);
        }
        rafId = requestAnimationFrame(loop);

      } catch (err: unknown) {
        const msg = err instanceof Error ? err.message : String(err);
        setError(`WebGPU init failed: ${msg}`);
      }
    }

    bootstrap();

    return () => {
      disposed = true;
      cancelAnimationFrame(rafId);
      engineRef.current?.free();
      engineRef.current = null;
    };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return { engineRef, isLoaded, error };
}
