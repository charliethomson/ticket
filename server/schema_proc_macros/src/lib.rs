extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse, parse_macro_input, DeriveInput};

const FIELD_DELIM: &str = "#$++,";
const ITEM_DELIM: &str = "$!@;";
const TABLE_MARKER: &str = "$%^$#$!$@#";
const PADDING_VALUE: &str = "$%&&#$*@@";

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
                let key = field.ident.unwrap();
                let value = if !field.attrs.is_empty() {
                    let attr: &syn::Attribute = field.attrs.get(0).unwrap();
                    get_db_name(attr)
                } else {
                    format!("{}", key.clone())
                };
                map.insert(key, value);
            }
        }
        _ => unreachable!("Why did you try to derive on not a struct lol"),
    }

    let for_body = map.iter().map(|(struct_name, db_field)| quote!{
        if let Some(__value) = self.#struct_name.clone() {
            ____strs__.push(format!(
                "{}{}{}\"{}{}{}\"",
                #TABLE_MARKER, #db_field, #FIELD_DELIM, #PADDING_VALUE, __value.to_string(), #PADDING_VALUE,
            ));
        }
    }).fold(TokenStream2::new(), |mut ret, cur_ts| {ret.extend(cur_ts.into_iter()); ret});

    let db_table: String = match format!("{}", struct_ident).as_str() {
        "WorkorderOptions" => "workorders.",
        "DeviceOptions" => "devices.",
        "StoreOptions" => "stores.",
        "CustomerOptions" => "customers.",
        "UserOptions" => "users.",
        "NotesOptions" => "notes.",
        _ => unreachable!("Only derive on the Options structs, if there's a new one, don't forget to add it's table to schema_proc_macros")
    }
    .to_owned();

    let gen = quote! {
        impl IntoDelimited for #struct_ident {
            fn into_delimited(&self) -> String {
                let mut ____strs__ = Vec::new();

                #for_body

                ____strs__.join(#ITEM_DELIM)
            }

            fn into_filter(&self) -> String {
                self.into_delimited()
                    .replace(#ITEM_DELIM, " and ")
                    .replace(#FIELD_DELIM, " like ")
                    .replace(#PADDING_VALUE, "%")
                    .replace(#TABLE_MARKER, #db_table)
            }
        }
    };
    gen.into()
}
