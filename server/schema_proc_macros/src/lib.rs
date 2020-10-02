extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse, parse_macro_input, DeriveInput};

const FIELD_DELIM: &'static str = "#$++,";
const ITEM_DELIM: &'static str = "$!@;";
const TABLE_MARKER: &'static str = "$%^$#$!$@#";
const PADDING_VALUE: &'static str = "$%&&#$*@@";

fn get_db_name(attr: &syn::Attribute) -> String {
    format!(
        "{}",
        parse::<TokenStream2>(TokenStream::from(attr.tokens.clone())).unwrap()
    )
    .split('"')
    .nth(1)
    .unwrap()
    .to_owned()
}

#[proc_macro_derive(IntoDelimited, attributes(db_name))]
pub fn derive_into_delimited(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_ident = input.ident;

    let mut map = std::collections::HashMap::new();

    let data = input.data;
    match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => {
            for field in named.into_iter() {
                let key = format!("{}", field.ident.unwrap());
                let value = if field.attrs.len() != 0 {
                    let attr: &syn::Attribute = field.attrs.iter().nth(0).unwrap();
                    get_db_name(attr)
                } else {
                    key.clone()
                };
                map.insert(key, value);
            }
        }
        _ => {}
    }

    let delimited: String = map
        .iter()
        .map(|(db_name, struct_name)| {
            format!(
                "{}{}{}\"{}{}{}\"",
                TABLE_MARKER, db_name, FIELD_DELIM, PADDING_VALUE, struct_name, PADDING_VALUE,
            )
        })
        .collect::<Vec<String>>()
        .join(ITEM_DELIM);

    let db_table: String = match format!("{}", struct_ident).as_str() {
        "WorkorderOptions" => "workorders.",
        "DeviceOptions" => "devices.",
        "StoreOptions" => "stores.",
        "CustomerOptions" => "customers.",
        "UserOptions" => "users.",
        "NotesOptions" => "notes.",
        _ => panic!("Only derive on the Options structs, if there's a new one, don't forget to add it's table to schema_proc_macros")
    }
    .to_owned();

    let gen = quote! {
        impl IntoDelimited for #struct_ident {
            fn into_delimited(&self) -> String {
                return stringify!(#delimited).to_owned()
            }

            fn into_filter(&self) -> String {
                self.into_delimited()
                .replace(#ITEM_DELIM, " and ")
                .replace(#FIELD_DELIM, " like ")
                .replace(#PADDING_VALUE, "%")
                .replace(#PADDING_VALUE, "%")
                .replace(#TABLE_MARKER, #db_table)
            }
        }
    };
    gen.into()
}
