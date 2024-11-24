use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn proc_macro_impl(args: TokenStream, ast: ItemStruct) -> TokenStream {
    let args = args.into_iter().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!("Invalid number of arguments: {:?}", args.len());
    }
    let opcode = &args[0];
    let opcode_sb = &args[2];

    let vis = &ast.vis;
    let ident = &ast.ident;

    quote! {
        #vis struct #ident {
            opcode: u8,
            opcode_sub: u8,
            rd: usize,
            rs1: usize,
            imm: u32,
        }

        impl From<u64> for #ident {
            fn from(raw: u64) -> Self {
                Self {
                    opcode: #opcode,
                    opcode_sub: #opcode_sb,
                    rd:  ((raw >>  8) &    0b11111) as usize,
                    rs1: ((raw >> 13) &      0b111) as usize,
                    imm: ((raw >> 18) & 0xffffffff) as u32,
                }
            }
        }
    }
}
