use proc_macro2::TokenStream;
use syn::DataStruct;
use syn::DeriveInput;

pub fn derive_hal(
    _attribute: &TokenStream,
    item: &DeriveInput,
    _struct_body: &DataStruct,
) -> TokenStream {
    let DeriveInput { ref ident, .. } = item;

    let ident_str = ident.to_string();

    // Rewrite decorated pin fields as `InputPin`, etc
    // Ignore non-decorated fields
    // Impl HalData trait to add a `new() -> Result<...>` fn to allocate resources and stuff
    // Impl Drop?
    // Impl Deref for some niceness? I should maybe do this outside this proc macro

    quote! {
        #item

        impl linuxcnc_hal::HalData for #ident {
            fn test_fn(&self) {
                println!("I'm a {}", #ident_str);
            }
        }
    }
}
