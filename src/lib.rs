//! Macro to automatically generate duck type interfaces with [wasm_bindgen](https://crates.io/crates/wasm-bindgen)
//! 
//! # Usage
//! 
//! The `[wasm_bindgen_duck_type]` attribute macro can be applied to structs to generate the duck type interface for the struct.
//! 
//! ## Example
//! 
//! JavaScript:
//! ```javascript
//! function MyFun() {
//!     return {
//!         number: 20,
//!         fun: () => {
//!             console.log("Hello, World!")
//!         }
//!     };
//! }
//! 
//! function GetNumber(input) {
//!     return input.number
//! }
//! ```
//! 
//! Rust:
//! ```no_run
//! use wasm_bindgen_duck_type::wasm_bindgen_duck_type;
//! use wasm_bindgen::prelude::*;
//! 
//! #[wasm_bindgen]
//! extern "C" {
//!     fn MyFun() -> MyType;
//!     fn GetNumber(input: MyType) -> i32;
//! }
//! 
//! #[wasm_bindgen_duck_type]
//! struct MyType {
//!     number: i32,
//!     fun: js_sys::Function,
//! }
//! 
//! fn main() {
//!     let result = MyFun();
//! 
//!     result.fun().call0(&JsValue::NULL);
//!     result.set_number(10);
//! 
//!     let input = MyType::default(); // All fields are null / default
//!     let input = MyType::new(
//!         42,
//!         JsValue::NULL.into() // Pass a function
//!     );
//! 
//!     let num = GetNumber(input); // 42
//! }
//! ```

extern crate wasm_bindgen_duck_type_macro_impl;

pub use wasm_bindgen_duck_type_macro_impl::*;
