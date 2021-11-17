use core::panic;

use proc_macro::TokenStream;

extern crate proc_macro;

use syn::{DeriveInput, Visibility, ext::IdentExt, parse_macro_input};
use quote::{ToTokens, quote};

#[proc_macro_derive(TesTes)]
pub fn a(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let target_name = item.ident.into_token_stream();
    let data = item.data;
    let code: TokenStream = match &data {
        syn::Data::Struct(str) => {
        
            let field_convert = str
                .fields
                .iter()
                .filter(|f| matches!(f.vis,Visibility::Public(_)))
                .map(|f| {
                    let name = f.ident.clone().unwrap().into_token_stream();
                    let type_name = match &f.ty {
                        syn::Type::Path(path) =>{
                            path.path.get_ident().clone().unwrap().into_token_stream()
                        },
                        _ => {
                            panic!("Unexpected Type: {:?}",f.ty);
                        }
                    };
                    quote! {
                        (stringify!(#name).to_string(),<Schema as SchemaNew<#type_name>>::new()),
                    }
                })
                .fold(quote!(), |mut acc,cur| {
                    acc.extend(cur);
                    acc
                });
            let code = quote! {
                impl ToSchema for #target_name {
                    fn to_schema() -> Schema {
                        Schema::Struct(
                            [#field_convert].into()
                        )
                    }
                }
            };
            code.into()
        },
        syn::Data::Enum(enm) => todo!(),
        syn::Data::Union(_) => todo!(),
    };
    code
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
