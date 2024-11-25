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
            rd: usize,
            rs1: usize,
            rs2: usize,
            imm: i32,
        }

        impl From<u64> for #ident {
            fn from(raw: u64) -> Self {
                let sext = |value: u32| {
                    if (value >> 24) & 1 == 1{
                        let extended_value = (value as u32) | 0xFF000000;
                        extended_value as i32
                    } else {
                        value as i32
                    }
                };

                assert_eq!(((raw >> 0) & 0b11111) as u8, #opcode);
                assert_eq!(((raw >> 5) & 0b111) as u8, #opcode_sb);
                Self {
                    rd:       ((raw >>  8) &    0b11111) as usize,
                    rs1:      ((raw >> 13) &    0b11111) as usize,
                    rs2:      ((raw >> 18) &    0b11111) as usize,
                    imm: sext(((raw >> 23) &  0x1ffffff) as u32),
                }
            }
        }
    }
}
