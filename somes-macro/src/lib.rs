mod composite_type;

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

#[proc_macro_derive(CompositeType)]
pub fn derive_create_pg_composite(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let type_name = name.to_string();
    let sql_type_name = type_name.to_snake_case();

    let fields = match input.data {
        Data::Struct(ds) => ds.fields,
        _ => {
            return syn::Error::new_spanned(
                name,
                "CreatePgComposite can only be derived for structs",
            )
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
            return syn::Error::new_spanned(
                name,
                "CreatePgComposite needs a struct with named fields",
            )
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
