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

fn get_map(
    named: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> std::collections::HashMap<proc_macro2::Ident, String> {
    let mut map = std::collections::HashMap::new();
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
    map
}

#[proc_macro_derive(Options, attributes(db_name))]
pub fn derive_options(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;
    let data = input.data;
    let map = match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => get_map(named),
        _ => unreachable!("Why did you try to derive on not a struct lol"),
    };

    let for_body = map.iter().map(|(struct_name, db_field)| quote!{
        if let Some(__value) = self.#struct_name.clone() {
            ____strs__.push(format!(
                "{}{}{}\"{}{}{}\"",
                #TABLE_MARKER, #db_field, #FIELD_DELIM, #PADDING_VALUE, __value.to_string(), #PADDING_VALUE,
            ));
        }
    }).fold(TokenStream2::new(), |mut ret, cur_ts| {ret.extend(cur_ts.into_iter()); ret});

    let db_table: String = match format!("{}", struct_ident).as_str() {
        "WorkorderOptions" => "workorders",
        "DeviceOptions" => "devices",
        "StoreOptions" => "stores",
        "CustomerOptions" => "customers",
        "UserOptions" => "users",
        "NotesOptions" => "notes",
        _ => unreachable!("Only derive on the Options structs, if there's a new one, don't forget to add it's table to schema_proc_macros")
    }
    .to_owned();

    let gen = quote! {
        impl Options for #struct_ident {
            fn into_delimited(&self) -> String {
                let mut ____strs__ = Vec::new();

                #for_body

                ____strs__.join(#ITEM_DELIM)
            }

            fn into_filter(&self) -> String {
                let mut table_spec = #db_table.to_owned();
                table_spec.push('.');
                self.into_delimited()
                    .replace(#ITEM_DELIM, " and ")
                    .replace(#FIELD_DELIM, " like ")
                    .replace(#PADDING_VALUE, "%")
                    .replace(#TABLE_MARKER, &table_spec)
            }

            fn into_update(&self) -> String {
                format!(
                    "update {} {} where {}.id={}",
                    #db_table,
                    format!(
                        "set {}",
                        self.into_delimited()
                            .replace(ITEM_DELIM, ", ")
                            .replace(FIELD_DELIM, "=")
                            .replace(PADDING_VALUE, "")
                            .replace(TABLE_MARKER, "")
                    ),
                    #db_table,
                    self.id.unwrap()
            )

            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Insert, attributes(db_name))]
pub fn derive_insert(input: TokenStream) -> TokenStream {
    // allow syn to parse the input
    let input = parse_macro_input!(input as DeriveInput);
    // get the ident for the struct
    let struct_ident = input.ident;
    // map the identifier used in the struct to the identifier used in the database
    let map = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => get_map(named),
        _ => unreachable!("Why did you try to derive on not a struct lol"),
    };

    // Get the database table name based on the struct name
    let db_table: String = match format!("{}", struct_ident).as_str() {
        "Workorder" | "WorkorderResponse" => "workorders",
        "Device" => "devices",
        "Store" => "stores",
        "Customer" => "customers",
        _ => unreachable!("Only derive on some structs, if there's a new one, don't forget to add it's table to schema_proc_macros")
    }
    .to_owned();

    // Generate the query string
    let query_str = format!(
        "insert into {} ({}) values ({});",
        db_table,
        map.values().cloned().collect::<Vec<String>>().join(", "),
        map.values()
            .map(|s| {
                let mut m = String::from(":");
                m.push_str(&s);
                m
            })
            .collect::<Vec<String>>()
            .join(", "),
    );
    let params_fields: TokenStream2 = map
        .iter()
        .map(|(ident, db_name)| {
            quote! {
                #db_name => self.#ident.clone(),
            }
        })
        .fold(TokenStream2::new(), |mut ret, cur_ts| {
            ret.extend(cur_ts.into_iter());
            ret
        });

    let gen = quote! {
        impl crate::db::Insert for #struct_ident {
            fn insert(&self) -> ::mysql::Result<Option<i64>> {
                use ::mysql::{params, prelude::Queryable};
                let mut ___conn = crate::db::get_connection()?;
                ___conn.exec_drop(
                    #query_str,
                    params! {
                        #params_fields
                    }
                )?;
                let ___pk_name__ = ___conn.exec_first::<String, String,  mysql::params::Params>(
                    format!("
                        select COLUMN_NAME
                        from information_schema.KEY_COLUMN_USAGE 
                        where 
                            TABLE_NAME='{}' 
                            and CONSTRAINT_NAME='PRIMARY';", 
                        #db_table),
                    mysql::params::Params::Empty)?
                    .unwrap();
                Ok(___conn.query_first::<i64, String>(
                    format!("SELECT max(LAST_INSERT_ID({})) FROM {}", ___pk_name__, #db_table),
                )?)
            }
        }
    };
    gen.into()
}
