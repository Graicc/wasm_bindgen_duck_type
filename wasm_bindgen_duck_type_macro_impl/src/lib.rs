//! Implementation for [wasm_bindgen_duck_type](https://docs.rs/wasm_bindgen_duck_type)

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

/// Attribute macro to generate duck type interface
#[proc_macro_attribute]
pub fn wasm_bindgen_duck_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();

    impl_duck_type(&ast)
}

fn impl_duck_type(item_struct: &syn::ItemStruct) -> TokenStream {
    let name = &item_struct.ident;
    let vis = &item_struct.vis;
    let fields = &item_struct.fields;
    if let syn::Fields::Named(feilds) = fields {
        let field_methods = feilds.named.iter().map(|x| {
            let f_name = x.ident.as_ref().unwrap();
            let f_type = &x.ty;

            let f_setter_name = format_ident!("set_{}", f_name);

            quote! {
                #[::wasm_bindgen::prelude::wasm_bindgen(method, getter)]
                #vis fn #f_name(this: &#name) -> #f_type;
                #[::wasm_bindgen::prelude::wasm_bindgen(method, setter)]
                #vis fn #f_setter_name(this: &#name, val: #f_type);
            }
        });

        let signature = feilds.named.iter().map(|x| {
            let f_name = x.ident.as_ref().unwrap();
            let f_type = &x.ty;

            quote! {
                #f_name: #f_type
            }
        });

        let new_impl = feilds.named.iter().map(|x| {
            let f_name = x.ident.as_ref().unwrap();
            let prop = format_ident!("set_{}", f_name);

            quote! {
                new.#prop(#f_name);
            }
        });

        let gen = quote! {
            #[::wasm_bindgen::prelude::wasm_bindgen(method, getter)]
            extern "C" {
                #vis type #name;

                #(#field_methods)*
            }

            impl #name {
                pub fn new(#(#signature),*) -> Self {
                    let new: Self = Self::default();

                    #(#new_impl)*

                    new
                }
            }

            impl ::std::default::Default for #name {
                fn default() -> Self {
                    JsValue::from(js_sys::Object::new()).into()
                }
            }
        };

        gen.into()
    } else {
        panic!("Expand macro can only be used on named structs");
    }
}
