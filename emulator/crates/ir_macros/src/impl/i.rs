use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn proc_macro_impl(_args: TokenStream, ast: ItemStruct) -> TokenStream {
    let vis = &ast.vis;
    let ident = &ast.ident;

    quote! {
        #vis struct #ident {
            opcode: u8,
            opcode_sub: u8,
            rd: u8,
            rs1: u8,
            imm: u32,
        }

        impl From<u64> for #ident {
            fn from(raw: u64) -> Self {
                Self {
                    opcode:     ((raw >>  0) &    0b11111) as u8,
                    opcode_sub: ((raw >>  5) &      0b111) as u8,
                    rd:         ((raw >>  8) &    0b11111) as u8,
                    rs1:        ((raw >> 13) &      0b111) as u8,
                    imm:        ((raw >> 18) & 0xffffffff) as u32,
                }
            }
        }
    }
}
