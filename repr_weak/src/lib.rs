use proc_macro::{TokenStream};
use syn::{parse_macro_input};
use quote::ToTokens;

#[proc_macro_attribute]
pub fn repr_weak(attr: TokenStream, item: TokenStream) -> TokenStream {
    let repr = parse_macro_input!(attr as syn::Type);
    let en = parse_macro_input!(item as syn::ItemEnum);

    let fields = en.variants.iter().filter_map(|x| {
        x.discriminant
            .as_ref()
            .map(|(_, expr)| (&x.ident, expr))
    });

    let statements = fields.map(|(id, expr)| {
        let statement: syn::ItemConst = syn::parse_quote!(pub const #id: #repr = #expr;);
        statement
    });

    let vis = &en.vis;

    let enum_name = &en.ident;

    let mut module: syn::ItemMod = syn::parse_quote!(
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        #vis mod #enum_name {

        }
    );

    for stmt in statements {
        if let Some((_, items)) = module.content.as_mut() {
            items.push(syn::Item::Const(stmt));
        }
    }

    module.into_token_stream().into()
}