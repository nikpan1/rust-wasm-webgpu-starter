import { useRef } from 'react';
import { GpuCanvas }    from './components/GpuCanvas';
import { ControlPanel } from './components/ControlPanel';
import { useWasmEngine } from './hooks/useWasmEngine';

function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { engineRef, isLoaded, error } = useWasmEngine(canvasRef);

  return (
    <div style={{ height: '100vh', margin: 0, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', gap: 12 }}>
      <GpuCanvas
        canvasRef={canvasRef}
        isLoaded={isLoaded}
        error={error}
      />

      <ControlPanel
        engineRef={engineRef}
        isLoaded={isLoaded}
      />
    </div>
  );
}

export default App;
