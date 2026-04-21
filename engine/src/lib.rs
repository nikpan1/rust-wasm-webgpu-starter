mod geometry;
mod math;
mod state;

use state::State;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// The public Wasm interface.
///
/// JS usage:
/// ```js
/// const app = await WasmApp.create(canvas);
/// app.render();          // called every requestAnimationFrame
/// app.increase_speed();  // called on button click
/// ```
#[wasm_bindgen]
pub struct WasmApp {
    state: State,
}

#[wasm_bindgen]
impl WasmApp {
    /// Asynchronously initialise WebGPU and return a ready-to-render `WasmApp`.
    /// Returns a `Promise<WasmApp>` in JavaScript.
    pub async fn create(canvas: HtmlCanvasElement) -> Result<WasmApp, JsValue> {
        console_error_panic_hook::set_once();

        let state = State::new(canvas)
            .await
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(WasmApp { state })
    }

    /// Advance the rotation angle, update the uniform buffer, and draw one frame.
    pub fn render(&mut self) {
        self.state.tick();
        let _ = self.state.render_frame();
    }

    /// Increase rotation speed on every button press.
    pub fn increase_speed(&mut self) {
        self.state.rotation_speed += 0.01;
    }

    /// Expose current speed so the UI can display it.
    pub fn rotation_speed(&self) -> f32 {
        self.state.rotation_speed
    }
}
