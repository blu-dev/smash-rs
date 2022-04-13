use proc_macro::{TokenStream};
use syn::{parse_macro_input, token::*, spanned::Spanned};
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

fn bare_fn_to_fn(attrs: &Vec<syn::Attribute>, name: &syn::Ident, public: &syn::Visibility, bare: &syn::TypeBareFn) -> Result<syn::ItemFn, syn::Error> {
    // basic idea:
    // 1. Get the argument named "this". This argument is going to become the self argument, and we base
    //      the mutability of the function on whether or not this is &Type or &mut Type

    let mut self_arg = syn::FnArg::Receiver(syn::Receiver {
        attrs: vec![],
        reference: Some((And(bare.fn_token.span()), None)),
        mutability: Some(Mut(bare.fn_token.span())),
        self_token: SelfValue(bare.fn_token.span())
    });

    let mut found_this = false;

    let mut remaining_args: syn::punctuated::Punctuated<&syn::BareFnArg, syn::token::Comma> = syn::punctuated::Punctuated::new();
    for arg in bare.inputs.iter() {
        let arg_name = match &arg.name {
            Some(name) => name,
            None => return Err(syn::Error::new(bare.span(), "This function must have named arguments"))
        };

        if arg_name.0.to_string() == "this" {
            let ty = &arg.ty;
            match ty {
                syn::Type::Reference(ref_type) => {
                    if let syn::FnArg::Receiver(self_arg) = &mut self_arg {
                        self_arg.mutability = ref_type.mutability.clone();
                    } else {
                        unreachable!()
                    }
                },
                _ => {
                    return Err(syn::Error::new(bare.span(), "This function must take a reference type named 'this'!"));
                }
            }
            found_this = true;
        } else {
            remaining_args.push(arg);
        }
    }

    if !found_this {
        return Err(syn::Error::new(bare.span(), "This function must take a reference type named 'this'!"));
    }

    let ret = &bare.output;

    let mut arg_names: syn::punctuated::Punctuated<&syn::Ident, syn::token::Comma> = syn::punctuated::Punctuated::new();
    for arg in remaining_args.iter() {
        arg_names.push(&arg.name.as_ref().unwrap().0);
    }

    let attrs = attrs.iter();

    Ok(syn::parse_quote!{
        #(#attrs)*
        #public fn #name(#self_arg, #remaining_args) #ret {
            unsafe {
                (self.vtable. #name)(self, #arg_names)
            }
        }
    })
}

#[proc_macro_attribute]
pub fn vtable_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ty_name = parse_macro_input!(attr as syn::Ident);
    let item = parse_macro_input!(item as syn::ItemStruct);

    let funcs: Vec<(&Vec<syn::Attribute>, &syn::Ident, &syn::Visibility, &syn::TypeBareFn)> = item.fields.iter().filter_map(|field| {
        if field.ident.is_none() {
            return None;
        }
        
        match &field.ty {
            syn::Type::BareFn(bare_fn) => Some((&field.attrs, field.ident.as_ref().unwrap(), &field.vis, bare_fn)),
            _ => None
        }
    }).collect();

    let mut new_funcs = vec![];
    for (attrs, ident, vis, bare_fn) in funcs.into_iter() {
        match bare_fn_to_fn(attrs, ident, vis, bare_fn) {
            Ok(func) => new_funcs.push(func),
            Err(e) => return e.into_compile_error().into_token_stream().into()
        }
    }

    let funcs = new_funcs.iter();

    quote::quote!(
        #item

        impl #ty_name {
            #(#funcs)*
        }
    ).into()
}