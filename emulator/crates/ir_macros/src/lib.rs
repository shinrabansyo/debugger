mod r#impl;

use syn::{parse_macro_input, ItemStruct};

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn I_style(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = attr.into();
    let ast = parse_macro_input!(item as ItemStruct);
    r#impl::i::proc_macro_impl(args, ast).into()
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn S_style(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = attr.into();
    let ast = parse_macro_input!(item as ItemStruct);
    r#impl::s::proc_macro_impl(args, ast).into()
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn R_style(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = attr.into();
    let ast = parse_macro_input!(item as ItemStruct);
    r#impl::r::proc_macro_impl(args, ast).into()
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn B_style(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = attr.into();
    let ast = parse_macro_input!(item as ItemStruct);
    r#impl::b::proc_macro_impl(args, ast).into()
}
