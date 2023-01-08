use proc_macro::TokenStream;
use proc_macro_error::{abort, emit_error, proc_macro_error, abort_call_site};
use syn::{parse_macro_input, ItemFn, AttributeArgs, spanned::Spanned, Visibility, Type, ReturnType, FnArg, NestedMeta, Lit, Ident, Meta, Path};
use quote::quote;

struct Argument {
    struct_ident: Ident,
    response: Path
}

struct TraitImplMetadata {
    vis: Visibility,
    param: Type,
    impl_fn: ItemFn
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let Argument {
        struct_ident,
        response
    } = get_struct_ident(parse_macro_input!(attr as AttributeArgs));
    let TraitImplMetadata {
        vis,
        param,
        impl_fn
    } = build_trait_impl_fn(parse_macro_input!(item as ItemFn));

    let expanded = quote! {
        #vis struct #struct_ident; impl crate::services::Service for #struct_ident {
            type Parameter = #param;
            type Response = #response;

            #impl_fn
        }
    };

    expanded.into()
}

fn get_struct_ident(attr: AttributeArgs) -> Argument {
    let mut attr = attr.into_iter();

    let Some(NestedMeta::Lit(Lit::Str(name))) = attr.next() else {
        abort_call_site!("The name of the struct must be given using string literal: #[service(\"FooBarService\", ReturnType)]");
    };

    let Some(NestedMeta::Meta(Meta::Path(path))) = attr.next() else {
        abort_call_site!("Expeted return type after the name of the struct: #[service(\"FooBarService\", ReturnType)]");
    };

    if attr.next().is_some() {
        abort_call_site!("Expeted single identifier for the name of the struct, found more than two");
    };

    Argument {
        struct_ident: syn::Ident::new(&name.value(), name.span()),
        response: path,
    }
}

fn build_trait_impl_fn(item_fn: ItemFn) -> TraitImplMetadata {
    let mut impl_fn = item_fn.clone();
    let mut args = item_fn.sig.inputs.iter();

    let Some(FnArg::Typed(param_type)) = args.next() else {
        abort!(item_fn.sig.inputs.span(), "Expected one argument for the parameter, found nothing or receiver");
    };
    if let Some(excessive) = args.next() {
        emit_error!(excessive.span(), "Expected one argument for the parameter, found more");
    }

    let ReturnType::Type(_, _) = item_fn.sig.output else {
        abort!(item_fn.sig.ident.span(), "Expected a return type for the response");
    };

    impl_fn.vis = Visibility::Inherited;
    impl_fn.sig.ident = syn::Ident::new("serve", impl_fn.sig.ident.span());

    TraitImplMetadata {
        vis: item_fn.vis,
        param: *param_type.ty.clone(),
        impl_fn
    }
}
