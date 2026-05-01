use crate::impl_dipa;
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

pub(super) fn create_zst_impl(enum_or_struct_name: &Ident) -> TokenStream2 {
    impl_dipa(
        enum_or_struct_name,
        quote! {()},
        quote! {()},
        quote! {
            cumulo_dipa::CreatedDelta {
                delta: (),
                did_change: false
            }
        },
        quote! {},
    )
}
