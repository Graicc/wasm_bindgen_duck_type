# wasm_bindgen_duck_type

Macro to automatically generate duck type interfaces with [wasm_bindgen](https://crates.io/crates/wasm-bindgen)

## Why

For most cases, [Serde](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html) should be used to serialize and deserialize objects passed between JavaScript and Rust.
Additionally, if the JavaScript object is an instance of a class, this crate is not necessary, as the implementation of the class can be used.

However, some types, such as a web_sys::Element or js_sys::Function, cannot be serialized as text, so this approach will not work. In these cases, this macro can be used to provide a typed interface for interaction, when the [JavaScript object is untyped](https://rustwasm.github.io/wasm-bindgen/reference/accessing-properties-of-untyped-js-values.html).

## How

This crate generates a [Duck Typed interface](https://rustwasm.github.io/wasm-bindgen/reference/working-with-duck-typed-interfaces.html) based on the given struct definition, as well as a `new` constructor and `Default` trait implementation.

## Usage

The `[wasm_bindgen_duck_type]` attribute macro can be applied to structs to generate the duck type interface for the struct.

### Example

JavaScript:
```javascript
function MyFun() {
    return {
        number: 20,
        fun: () => {
            console.log("Hello, World!")
        }
    };
}

function GetNumber(input) {
    return input.number
}
```

Rust:
```rust
use wasm_bindgen_duck_type::wasm_bindgen_duck_type;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn MyFun() -> MyType;
    fn GetNumber(input: MyType) -> i32;
}

#[wasm_bindgen_duck_type]
struct MyType {
    number: i32,
    fun: js_sys::Function,
}

fn main() {
    let result = MyFun();

    result.fun().call0(&JsValue::NULL);
    result.set_number(10);

    let input = MyType::default(); // All fields are null / default
    let input = MyType::new(
        42,
        JsValue::NULL.into() // Pass a function
    );

    let num = GetNumber(input); // 42
}
```