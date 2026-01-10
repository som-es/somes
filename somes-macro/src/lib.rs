mod composite_type;
mod filter;

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

use crate::{composite_type::map_type_to_sql, filter::rebuild_type};

#[proc_macro_derive(CompositeType)]
pub fn derive_create_pg_composite(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let type_name = name.to_string();
    let sql_type_name = type_name.to_snake_case();

    let fields = match input.data {
        Data::Struct(ds) => ds.fields,
        _ => {
            return syn::Error::new_spanned(name, "CompositeType can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let mut cols = Vec::new();

    match fields {
        Fields::Named(named) => {
            for f in named.named.iter() {
                let ident = f.ident.as_ref().unwrap();
                let rust_field_name = ident.to_string();
                let (sql_ty, _nullable) = map_type_to_sql(&f.ty);
                cols.push((rust_field_name, sql_ty));
            }
        }
        Fields::Unnamed(_) | Fields::Unit => {
            return syn::Error::new_spanned(name, "CompositeType needs a struct with named fields")
                .to_compile_error()
                .into();
        }
    }

    let crate_name = proc_macro_crate::crate_name("somes-common-lib")
        .ok()
        .and_then(|res| match res {
            proc_macro_crate::FoundCrate::Itself => Some(quote!(crate)),
            proc_macro_crate::FoundCrate::Name(name) => {
                let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
                Some(quote!(::#ident))
            }
        })
        .unwrap_or_else(|| quote!(::somes_common_lib)); // fallback

    // build SQL fields string like: "field1 integer, field2 text"
    let cols_sql: Vec<String> = cols.iter().map(|(n, t)| format!("{} {}", n, t)).collect();
    let cols_sql_joined = cols_sql.join(", ");

    let cols_names = cols.iter().map(|c| &c.0);

    let tokens = quote! {
        impl #crate_name::ToCompositeType for #name {
            fn field_orders() -> Vec<&'static str> {
                vec![#(stringify!(#cols_names)),*]
            }
            fn type_name() -> &'static str {
                #sql_type_name
            }
            fn to_sql_create_composite_type() -> String {
                format!("CREATE TYPE {} AS ({});", #sql_type_name, #cols_sql_joined)
            }
        }
    };

    tokens.into()
}

/*
struct DecreeFilter {
    // Option<String>
    ministerial_issuer: Option<FilterOp<String>>,
    // Option<Vec<i32>>
    gov_official_ids: Option<FilterOp<Vec<i32>>>,
    // Option<Vec<Document>>
    documents: Option<Vec<Document>>
}
*/
#[proc_macro_derive(MeilisearchFilter, attributes(filter))]
pub fn derive_meilisearch_filter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let vis = input.vis;

    let fields = match input.data {
        Data::Struct(ds) => ds.fields,
        _ => {
            return syn::Error::new_spanned(
                name,
                "MeilisearchFilter can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut updated_fields = Vec::new();

    match fields {
        Fields::Named(named) => {
            for f in named.named.iter() {
                let mut has_make_vec = false;
                for attr in &f.attrs {
                    if attr.path().is_ident("filter") {
                        let parse_result = attr.parse_nested_meta(|meta| {
                            // Check if the token inside is 'make_vec'
                            if meta.path.is_ident("make_vec") {
                                has_make_vec = true;
                                Ok(())
                            } else {
                                // Handle other options inside #[filter(...)] if necessary
                                Ok(())
                            }
                        });

                        if let Err(e) = parse_result {
                            return e.to_compile_error().into();
                        }
                    }
                }

                let ident = f.ident.as_ref().unwrap();
                let rust_field_name = ident.clone();
                let rebuilt_type = rebuild_type(&f.ty, has_make_vec);
                updated_fields.push((rust_field_name, rebuilt_type));
            }
        }
        Fields::Unnamed(_) | Fields::Unit => {
            return syn::Error::new_spanned(
                name,
                "MeilisearchFilter needs a struct with named fields",
            )
            .to_compile_error()
            .into();
        }
    }

    let simple_filter_args = updated_fields.iter().flat_map(|(field_name, ty)| {
        if ty.unrecognized_ident.is_some() {
            return None;
        }
        let field_name_str = field_name.to_string();
        Some(quote! { self.#field_name.into_filter(#field_name_str), })
    });

    let filterable_fields = updated_fields.iter().flat_map(|(field_name, ty)| {
        if ty.unrecognized_ident.is_some() {
            return None;
        }
        let field_name = field_name.to_string();
        Some(quote! { #field_name, })
    });

    let updated_fields = updated_fields.iter().map(|(field_name, ty)| {
        let type_tokens = &ty.updated_path;
        quote! {
            #vis #field_name: #type_tokens,
        }
    });

    let filter_name = format_ident!("{}Filter", name);

    let tokens = quote! {
        #[derive(Debug, Serialize, Deserialize)]
        #vis struct #filter_name {
            #( #updated_fields )*
        }

        impl #filter_name {
            #vis fn filter_arguments(self) -> Vec<Option<FilterArgument>> {
                use somes_meilisearch_filter::IntoFilterArgument;
                vec![#( #simple_filter_args )*]
            }

            #vis fn filterable_fields() -> Vec<&'static str> {
                vec![#( #filterable_fields )*]
            }
        }
    };

    tokens.into()
}
