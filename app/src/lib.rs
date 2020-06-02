use wasm_bindgen::prelude::wasm_bindgen;

use asteroids::Controls;

pub mod render;
use render::PathList;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct App(asteroids::Game);

#[wasm_bindgen]
impl App {
    pub fn new() -> Self {
        console_log!("Hello {}!", "world");
        App(asteroids::Game::new())
    }

    pub fn step(&mut self, dt: f64, input: u32) -> () {
        if dt <= 0.0 {
            return ();
        }
        self.0.step(dt, Controls::new(input))
    }

    pub fn render(&self) -> PathList {
        let mut list = PathList::new();
        if let Some(player) = self.0.player() {
            render::player(player, &mut list);
        }
        render::asteroids(self.0.asteroids(), &mut list);
        render::blasts(self.0.blasts(), &mut list);
        render::particles(self.0.particles(), &mut list);
        render::polylines(self.0.text(), 1.0, &mut list);
        render::polylines(&self.0.hud(), 0.3, &mut list);
        list
    }
}
