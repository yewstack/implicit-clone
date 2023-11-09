use quote::quote;

#[proc_macro_derive(ImplicitClone)]
pub fn derive_implicit_clone(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::ItemStruct { ident, .. } = syn::parse_macro_input!(item as syn::ItemStruct);
    let res = quote! {
        impl ::implicit_clone::ImplicitClone for #ident {}
    };
    res.into()
}
