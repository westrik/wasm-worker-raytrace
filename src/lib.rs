extern crate wasm_bindgen;
use rampage::rampage::{render_scene,RenderConfig};

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    let render_config = RenderConfig {
        num_threads: 1,
        num_iterations: 40,
        max_depth: 20,
        dimensions: (300, 250),
    };
    
    let _rendered_scene = render_scene("random_spheres", render_config);
 
    "render succeeded".to_string()
}
