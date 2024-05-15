// wasm-pack uses wasm-bindgen to provide a bridge between the types of JavaScript and Rust
// it allows JavaScript to call a Rust API with a string
// or a Rust function to catch a JavaScript exception
use wasm_bindgen::prelude::*;

// build command for a wasm package that specifies web use
// wasm-pack build --target web

// callig the wasm_bindgen attribute that modifys the code that follows
#[wasm_bindgen]
// extern tells Rust that we want to call externally defined functions
extern {
    // this is a Rust function but it's also the alert function provided by JavaScript
    pub fn alert(s: &str);
}

#[wasm_bindgen]
// producing a Rust functions that JavaScript can call
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}