extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[proc_macro_derive(DefaultID)]
pub fn defaultID_derive(input:TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote!{
        impl DefaultID for #name {
            fn set_default_id(&mut self) {
                let start = SystemTime::now();            
                let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap_or_default();
                let mills = since_the_epoch.as_mills(); // 返回自 epoch 以来的毫秒数

                self.id = "";
            }
        }
    };

    TokenStream::from(expanded)
}

