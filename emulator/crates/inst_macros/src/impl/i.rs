use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn proc_macro_impl(args: TokenStream, ast: ItemStruct) -> TokenStream {
    let args = args.into_iter().collect::<Vec<_>>();
    if args.len() != 1 {
        panic!("Invalid number of arguments: {:?}", args.len());
    }
    let opcode = &args[0];

    let vis = &ast.vis;
    let ident = &ast.ident;

    quote! {
        #vis struct #ident {
            rd: usize,
            rs1: usize,
            imm: i32,
        }

        impl From<u64> for #ident {
            fn from(raw: u64) -> Self {
                assert_eq!(((raw >> 32) & 0b111111) as u8, #opcode);
                Self {
                    rd:  ((raw >> 43) & 0b11111) as usize,
                    rs1: ((raw >> 38) & 0b11111) as usize,
                    imm: (raw & 0xffffffff) as i32,
                }
            }
        }
    }
}
