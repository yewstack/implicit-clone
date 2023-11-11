use quote::quote;

#[proc_macro_derive(ImplicitClone)]
pub fn derive_implicit_clone(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput {
        ident, generics, ..
    } = syn::parse_macro_input!(item as syn::DeriveInput);
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let generics = generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Type(syn::TypeParam {
                attrs,
                ident,
                colon_token: _,
                bounds,
                eq_token,
                default,
            }) => {
                let bounds = bounds
                    .iter()
                    .map(|bound| quote! { #bound })
                    .chain(std::iter::once(quote! { ::implicit_clone::ImplicitClone }))
                    .collect::<Vec<_>>();
                quote! {
                    #(#attrs)* #ident: #(#bounds)+* #eq_token #default
                }
            }
            _ => quote! { #param },
        })
        .collect::<Vec<_>>();
    let generics = if generics.is_empty() {
        quote! {}
    } else {
        quote! {
            <#(#generics),*>
        }
    };
    let res = quote! {
        impl #generics ::implicit_clone::ImplicitClone for #ident #ty_generics #where_clause {}
    };
    res.into()
}
