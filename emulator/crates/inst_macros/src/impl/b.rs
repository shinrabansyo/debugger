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
            rs2: usize,
            imm: i32,
        }

        impl From<u64> for #ident {
            fn from(raw: u64) -> Self {
                let sext = |value: u32| {
                    if (value >> 26) & 1 == 1 {
                        let extended_value = value | 0xF8000000;
                        extended_value as i32
                    } else {
                        value as i32
                    }
                };

                assert_eq!(((raw >> 32) & 0b111111) as u8, #opcode);
                Self {
                    rd:  ((raw >> 43) & 0b11111) as usize,
                    rs1: ((raw >> 38) & 0b11111) as usize,
                    rs2: ((raw >> 27) & 0b11111) as usize,
                    imm: sext((raw & 0x7ffffff) as u32),
                }
            }
        }
    }
}
