//! Proc macro derives for LinuxCNC

#![deny(missing_docs)]

#[macro_use]
extern crate quote;
extern crate proc_macro;

mod derive_hal;

use derive_hal::derive_hal;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput};

/// Add functionality to a struct to allow it to be used to store HAL pin/parameter/signal values
#[proc_macro_attribute]
pub fn hal(attribute: TokenStream, item: TokenStream) -> TokenStream {
    println!("{:?}", attribute.to_string());
    // let attribute: DeriveInput = syn::parse(attribute).unwrap();
    let item: DeriveInput = syn::parse(item).unwrap();

    expand_derive_event_create(attribute, item).into()
}

fn expand_derive_event_create(attribute: TokenStream, item: DeriveInput) -> TokenStream {
    match item.data {
        Data::Struct(ref body) => derive_hal(&attribute.into(), &item, &body).into(),
        _ => panic!("HAL macro can only be derived on structs"),
    }
}
