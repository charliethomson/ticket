use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse, parse_macro_input, DeriveInput};

const FIELD_DELIM: &str = "#$++,";
const ITEM_DELIM: &str = "$!@;";
const TABLE_MARKER: &str = "$%^$#$!$@#";
const PADDING_VALUE: &str = "$%&&#$*@@";
const COLLECTION_MARKER: &str = "**#$*$*#(@$!@";

fn get_types(data: &syn::Data) -> Vec<(proc_macro2::Ident, syn::Type)> {
    let mut map = vec![];
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = data.to_owned()
    {
        let mut iter = named.into_iter();
        while let Some(syn::Field {
            ident: Some(ident),
            ty,
            ..
        }) = iter.next()
        {
            map.push((ident, ty));
        }
    }

    map
}

fn is_vec(ty: &syn::Type) -> bool {
    match ty.to_owned() {
        syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => match segments.iter().next() {
            Some(syn::PathSegment { ident, arguments })
                if ident == &syn::Ident::new("Option", proc_macro2::Span::call_site()) =>
            {
                match arguments {
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }) => match args.iter().next() {
                        Some(syn::GenericArgument::Type(ty2)) => is_vec(ty2),
                        _ => false,
                    },
                    _ => unreachable!(),
                }
            }

            Some(syn::PathSegment { ident, .. })
                if ident == &syn::Ident::new("Vec", proc_macro2::Span::call_site()) =>
            {
                true
            }
            _ => false,
        },
        _ => false,
    }
}

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
) -> im::hashmap::HashMap<proc_macro2::Ident, String> {
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
    map.into()
}

// #[proc_macro_derive(Options, attributes(db_name))]
// pub fn derive_options(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_ident = input.ident;
//     let data = input.data;
//     let map = match data.to_owned() {
//         syn::Data::Struct(syn::DataStruct {
//             fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
//             ..
//         }) => get_map(named),
//         _ => unreachable!("Why did you try to derive on not a struct lol"),
//     };

//     let combo_map: im::hashmap::HashMap<proc_macro2::Ident, (String, syn::Type)> =
//         map.intersection_with(get_types(&data).into(), |db_field, ty| (db_field, ty));

//     let get_value = quote! {
//         match format!("{:?}", __value).as_str() {
//             "true" => "1",
//             "false" => "0",
//             e => &e
//         }
//     };

//     let for_body = combo_map.iter().map(|(struct_name, (db_field, ty))| {
//         if is_vec(&ty) {
//             quote!{
//                 if let Some(iterable) = self.#struct_name.clone() {
//                     ____strs__.push(iterable.iter().map(|__value| format!(
//                         "{}{}{}\"{}{}{}\"",
//                         #TABLE_MARKER, #db_field, #FIELD_DELIM, #PADDING_VALUE, #get_value, #PADDING_VALUE,
//                     )).collect::<Vec<String>>().join(#COLLECTION_MARKER));
//                 }
//             }
//         } else {
//             quote!{
//                 if let Some(__value) = self.#struct_name.clone() {
//                     ____strs__.push(format!(
//                         "{}{}{}\"{}{}{}\"",
//                         #TABLE_MARKER, #db_field, #FIELD_DELIM, #PADDING_VALUE, #get_value, #PADDING_VALUE,
//                     ));
//                 }
//             }
//         }
//     }).fold(TokenStream2::new(), |mut ret, cur_ts| {ret.extend(cur_ts.into_iter()); ret});

//     let db_table: String = match format!("{}", struct_ident).as_str() {
//         "WorkorderOptions" => "workorders",
//         "DeviceOptions" => "devices",
//         "StoreOptions" => "stores",
//         "CustomerOptions" => "customers",
//         "UserOptions" => "users",
//         "NotesOptions" => "notes",
//         _ => unreachable!("Only derive on the Options structs, if there's a new one, don't forget to add it's table to schema_proc_macros")
//     }
//     .to_owned();

//     // TODO: Implement some way of returning a bad request if we try to "update"
//     //       with a Vec type present
//     let gen = quote! {
//         impl Options for #struct_ident {
//             fn into_delimited(&self) -> String {
//                 let mut ____strs__ = Vec::new();

//                 #for_body

//                 ____strs__.join(#ITEM_DELIM)
//             }

//             fn into_filter(&self) -> String {
//                 let mut table_spec = #db_table.to_owned();
//                 table_spec.push('.');
//                 self.into_delimited()
//                     .replace(#ITEM_DELIM, " and ")
//                     .replace(#FIELD_DELIM, " like ")
//                     .replace(#PADDING_VALUE, "%")
//                     .replace(#TABLE_MARKER, &table_spec)
//                     .replace(#COLLECTION_MARKER, " or ")
//             }

//             fn into_update(&self) -> String {
//                 format!(
//                     "update {} {} where {}.id={}",
//                     #db_table,
//                     format!(
//                         "set {}",
//                         self.into_delimited()
//                             .replace(#ITEM_DELIM, ", ")
//                             .replace(#FIELD_DELIM, "=")
//                             .replace(#PADDING_VALUE, "")
//                             .replace(#TABLE_MARKER, "")
//                     ),
//                     #db_table,
//                     self.id.unwrap()
//             )

//             }
//         }
//     };
//     gen.into()
// }

// #[proc_macro_derive(Insert, attributes(db_name))]
// pub fn derive_insert(input: TokenStream) -> TokenStream {
//     // allow syn to parse the input
//     let input = parse_macro_input!(input as DeriveInput);
//     // get the ident for the struct
//     let struct_ident = input.ident;
//     // map the identifier used in the struct to the identifier used in the database
//     let map = match input.data {
//         syn::Data::Struct(syn::DataStruct {
//             fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
//             ..
//         }) => get_map(named),
//         _ => unreachable!("Why did you try to derive on not a struct lol"),
//     };

//     // TODO: Remove

//     // Get the database table name based on the struct name
//     let db_table: String = match format!("{}", struct_ident).as_str() {
//         "Workorder" | "WorkorderResponse" => "workorders",
//         "Device" => "devices",
//         "Store" => "stores",
//         "Customer" => "customers",
//         _ => unreachable!("Only derive on some structs, if there's a new one, don't forget to add it's table to schema_proc_macros")
//     }
//     .to_owned();

//     // Generate the query string
//     let query_str = format!(
//         "insert into {} ({}) values ({});",
//         db_table,
//         map.values().cloned().collect::<Vec<String>>().join(", "),
//         map.values()
//             .map(|s| {
//                 let mut m = String::from(":");
//                 m.push_str(&s);
//                 m
//             })
//             .collect::<Vec<String>>()
//             .join(", "),
//     );
//     let params_fields: TokenStream2 = map
//         .iter()
//         .map(|(ident, db_name)| {
//             quote! {
//                 #db_name => self.#ident.clone(),
//             }
//         })
//         .fold(TokenStream2::new(), |mut ret, cur_ts| {
//             ret.extend(cur_ts.into_iter());
//             ret
//         });

//     let gen = quote! {
//         impl crate::db::Insert for #struct_ident {
//             fn insert(&self) -> ::mysql::Result<Option<i64>> {
//                 use ::mysql::{params, prelude::Queryable};
//                 let mut ___conn = crate::db::get_connection()?;
//                 ___conn.exec_drop(
//                     #query_str,
//                     params! {
//                         #params_fields
//                     }
//                 )?;
//                 let ___pk_name__ = ___conn.exec_first::<String, String,  mysql::params::Params>(
//                     format!("
//                         select COLUMN_NAME
//                         from information_schema.KEY_COLUMN_USAGE
//                         where
//                             TABLE_NAME='{}'
//                             and CONSTRAINT_NAME='PRIMARY';",
//                         #db_table),
//                     mysql::params::Params::Empty)?
//                     .unwrap();
//                 Ok(___conn.query_first::<i64, String>(
//                     format!("SELECT max(LAST_INSERT_ID({})) FROM {}", ___pk_name__, #db_table),
//                 )?)
//             }
//         }
//     };
//     gen.into()
// }

#[proc_macro_attribute]
pub fn build_tuple(_: TokenStream, input: TokenStream) -> TokenStream {
    let original_input = TokenStream2::from(input.clone());
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;
    let tuple_ty = syn::Ident::new(
        &format!("{}Tuple", struct_ident),
        proc_macro2::Span::call_site(),
    );

    let map = get_types(&input.data);

    let parts = map.iter().fold(TokenStream2::new(), |mut ret, (_, ty)| {
        ret.extend(quote! {
            #ty,
        });
        ret
    });
    let names = map.iter().fold(TokenStream2::new(), |mut ret, (ident, _)| {
        ret.extend(quote! {
            #ident,
        });
        ret
    });

    let gen = quote! {
        pub type #tuple_ty = (#parts);

        #original_input

        impl From<#tuple_ty> for #struct_ident {
            fn from(tuple: #tuple_ty) -> #struct_ident {
                let (#names) = tuple;
                #struct_ident {
                    #names
                }
            }
        }
    };
    gen.into()
}

fn get_builder(types: Vec<(proc_macro2::Ident, syn::Type)>) -> TokenStream2 {
    let builder_functions = types
        .iter()
        .map(|(ident, ty)| {
            quote! {
                pub fn #ident(mut self, val: #ty) -> Self {
                    self.#ident = Some(val);
                    self
                }
            }
        })
        .fold(TokenStream2::new(), |mut acc, cur| {
            acc.extend(cur);
            acc
        });

    let new_body = types
        .iter()
        .map(|(ident, _)| {
            quote! {
                #ident: None,
            }
        })
        .fold(TokenStream2::new(), |mut acc, cur| {
            acc.extend(cur);
            acc
        });
    let gen = quote! {
            pub fn new() -> Self {
                Self {
                    #new_body
                }
            }
            #builder_functions
    };
    gen.into()
}

#[proc_macro_derive(Options, attributes(db_name))]
pub fn derive_options(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;
    let data = input.data;

    let types = get_types(&data);
    let options_ident = syn::Ident::new(
        &format!("{}Options", struct_ident.to_string()),
        proc_macro2::Span::call_site(),
    );
    let options_fields = types
        .iter()
        .map(|(ident, ty)| {
            quote! {
                #ident: Option<#ty>,
            }
        })
        .fold(TokenStream2::new(), |mut acc, cur| {
            acc.extend(cur);
            acc
        });
    let builder = get_builder(types);

    let gen = quote! {
        #[derive(Filter)]
        pub struct #options_ident {
            #options_fields
        }
        impl #options_ident {
            #builder
        }

        impl crate::db::Exists for #options_ident {
            fn exists(&self) -> Option<i64> {
                let query = format!("select id from {} where {}", #struct_ident::TABLE_NAME, self.into_filter())
            }
        }

        impl crate::db::HasTable for #options_ident {
            use crate::db::HasTable;
            const TABLE_NAME: &'static str = #struct_ident::TABLE_NAME;
        }

    };
    println!("{}", gen.to_string());
    gen.into()
}

#[proc_macro_derive(Filter)]
pub fn derive_filter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;
    let data = input.data;
    let map = match data.to_owned() {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => get_map(named),
        _ => unreachable!("Why did you try to derive on not a struct lol"),
    };
    let combo_map: im::hashmap::HashMap<proc_macro2::Ident, (String, syn::Type)> =
        map.intersection_with(get_types(&data).into(), |db_field, ty| (db_field, ty));
    let get_value = quote! {
        match format!("{:?}", __value).as_str() {
            "true" => "1",
            "false" => "0",
            e => &e
        }
    };
    let for_body = combo_map.iter().map(|(struct_name, (db_field, ty))| {
        if is_vec(&ty) {
            quote!{
                if let Some(iterable) = self.#struct_name.clone() {
                    ____strs__.push(iterable.iter().map(|__value| format!(
                        "{}{}{}\"{}{}{}\"",
                        #TABLE_MARKER, #db_field, #FIELD_DELIM, #PADDING_VALUE, #get_value, #PADDING_VALUE,
                    )).collect::<Vec<String>>().join(#COLLECTION_MARKER));
                }
            }
        } else {
            quote!{
                if let Some(__value) = self.#struct_name.clone() {
                    ____strs__.push(format!(
                        "{}{}{}\"{}{}{}\"",
                        #TABLE_MARKER, #db_field, #FIELD_DELIM, #PADDING_VALUE, #get_value, #PADDING_VALUE,
                    ));
                }
            }
        }
    }).fold(TokenStream2::new(), |mut ret, cur_ts| {ret.extend(cur_ts.into_iter()); ret});

    let gen = quote! {
        impl crate::db::Filter for #struct_ident {
            fn into_delimited(&self) -> String {
                let mut ____strs__ = Vec::new();

                #for_body

                ____strs__.join(#ITEM_DELIM)
            }

            fn into_filter(&self) -> String {
                let mut table_spec = self.TABLE_NAME.to_owned();
                table_spec.push('.');
                self.into_delimited()
                    .replace(#ITEM_DELIM, " and ")
                    .replace(#FIELD_DELIM, " like ")
                    .replace(#PADDING_VALUE, "%")
                    .replace(#TABLE_MARKER, &table_spec)
                    .replace(#COLLECTION_MARKER, " or ")
            }

            fn into_update(&self) -> String {
                format!(
                    "update {} {} where {}.id={}",
                    self.TABLE_NAME,
                    format!(
                        "set {}",
                        self.into_delimited()
                            .replace(#ITEM_DELIM, ", ")
                            .replace(#FIELD_DELIM, "=")
                            .replace(#PADDING_VALUE, "")
                            .replace(#TABLE_MARKER, "")
                    ),
                    self.TABLE_NAME,
                    self.id.unwrap()
                )
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn table_name(table_name: TokenStream, input: TokenStream) -> TokenStream {
    let original_input = TokenStream2::from(input.clone());
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;
    let table_name = table_name.to_string();
    let gen = quote! {
        #original_input

        impl crate::db::HasTable for #struct_ident {
            const TABLE_NAME: &'static str = #table_name;
        }
    };

    gen.into()
}
