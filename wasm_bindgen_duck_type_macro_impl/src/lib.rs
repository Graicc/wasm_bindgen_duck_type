extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn wasm_bindgen_duck_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();

    impl_duck_type(&ast)
}

fn impl_duck_type(item_struct: &syn::ItemStruct) -> TokenStream {
    let name = &item_struct.ident;
    let feilds = &item_struct.fields;
    if let syn::Fields::Named(feilds) = feilds {
        let feilds = feilds.named.iter().map(|x| {
            let f_name = x.ident.as_ref().unwrap();
            let f_type = &x.ty;

            quote! {
                #[wasm_bindgen::wasm_bindgen(method, getter)]
                pub fn #f_name(this: &#name) -> #f_type;
            }
        });

        let gen = quote! {
            #[wasm_bindgen::wasm_bindgen(method, getter)]
            extern "C" {
                pub type #name;

                #(#feilds)*
            }
        };

        gen.into()
    } else {
        panic!("Expand macro can only be used on named structs");
    }
}
