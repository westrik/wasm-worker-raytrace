//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use rampage::rampage::{render_scene, RenderConfig};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn render() {
    let render_config = RenderConfig {
        num_threads: 1,
        num_iterations: 100,
        max_depth: 60,
        dimensions: (400, 300),
    };

    let _rendered_scene = render_scene("random_spheres", render_config);
}
