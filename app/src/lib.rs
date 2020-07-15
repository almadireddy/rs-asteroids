extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use js_sys::{Float64Array, ArrayBuffer};
use asteroids::Controls;

pub mod render;
use render::PathList;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can  bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// println! -like macro that turns into JS console.log using above definition
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct App {
    game: asteroids::Game,
    ws: WebSocket,
}

#[wasm_bindgen]
impl App {
    pub fn new() -> Self {
        let ws = WebSocket::new("ws://localhost:8088/game/").unwrap();
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        let cloned_ws = ws.clone();

        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<ArrayBuffer>() {
                let array = Float64Array::new(&abuf);
                let len = array.byte_length() as usize;

                cloned_ws.set_binary_type(web_sys::BinaryType::Blob);

                match cloned_ws.send_with_array_buffer(&(Float64Array::new_with_length(3).buffer())) {
                    Ok(_) => console_log!("binary message successfully sent"),
                    Err(err) => console_log!("error sending message: {:?}", err),
                }
            }
            else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                console_log!("message event, received blob: {:?}", blob);

                // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
                let fr = web_sys::FileReader::new().unwrap();
                let fr_c = fr.clone();

                let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
                    let array = Float64Array::new(&fr_c.result().unwrap());
                    let len = array.byte_length() as usize;
                    console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
                }) as Box<dyn FnMut(web_sys::ProgressEvent)>);

                fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                fr.read_as_array_buffer(&blob).expect("blob not readable");
                onloadend_cb.forget();
            } 
            else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                console_log!("message event, received Text: {:?}", txt);
            } 
            else {
                console_log!("message event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            match cloned_ws.send_with_str("ping") {
                Ok(_) => console_log!("message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }

            match cloned_ws.send_with_array_buffer(&(Float64Array::new_with_length(3).buffer())) {
                Ok(_) => console_log!("binary message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(JsValue)>);

        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        App {
            game: asteroids::Game::new(),
            ws
        }
    }

    pub fn step(&mut self, dt: f64, input: u32) -> () {
        if dt <= 0.0 {
            return ();
        }
        
        self.game.step(dt, Controls::new(input))
    }

    pub fn render(&self) -> PathList {
        let mut list = PathList::new();

        if let Some(player) = self.game.player() {
            let floats = Float64Array::new_with_length(3);
            let place = &player.placement;
            floats.set_index(0, place.position.x);
            floats.set_index(1, place.position.y);
            floats.set_index(2, place.rotation);

            self.ws.send_with_array_buffer(&floats.buffer()).unwrap();
            render::player(player, &mut list);
        }

        render::asteroids(self.game.asteroids(), &mut list);
        render::blasts(self.game.blasts(), &mut list);
        render::particles(self.game.particles(), &mut list);
        render::polylines(self.game.text(), 1.0, &mut list);
        render::polylines(&self.game.hud(), 0.3, &mut list);

        list
    }
}
