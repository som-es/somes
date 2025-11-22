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

    // build SQL fields string like: "field1 integer, field2 text"
    let cols_sql: Vec<String> = cols.iter().map(|(n, t)| format!("{} {}", n, t)).collect();
    let cols_sql_joined = cols_sql.join(", ");

    let tokens = quote! {
        impl #name {
            pub fn to_sql_create_type() -> String {
                let create = format!("CREATE TYPE {} AS ({});", #sql_type_name, #cols_sql_joined);
                create
            }
        }
    };

    tokens.into()
}

fn map_type_to_sql(ty: &Type) -> (String, bool) {
    match ty {
        Type::Path(tp) => {
            if let Some(seg) = tp.path.segments.last() {
                let ident = seg.ident.to_string();
                if ident == "Option" {
                    // extract inner generic
                    if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                        if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                            let (s, _inner_nullable) = map_type_to_sql(inner_ty);
                            return (s, true);
                        }
                    }
                }
            }

            let last = tp.path.segments.last().unwrap();
            let ident = last.ident.to_string();
            let ident_snake_case = ident.to_snake_case();

            let sql = match ident.as_str() {
                "i8" | "i16" | "i32" | "isize" => "integer",
                "i64" => "bigint",
                "u8" | "u16" | "u32" | "u64" | "usize" => "bigint",
                "f32" => "real",
                "f64" => "double precision",
                "bool" => "boolean",
                "String" | "str" => "text",
                "Uuid" => "uuid",
                "Json" | "Value" => "jsonb",

                // chrono types
                "NaiveDate" => "date",
                "NaiveDateTime" => "timestamp",
                "DateTime" => "timestamptz",

                // bigdecimal
                "BigDecimal" => "numeric",

                // fallback
                _ => ident_snake_case.as_str(),
            };

            (sql.to_string(), false)
        }
        Type::Reference(tr) => map_type_to_sql(&tr.elem),
        Type::Array(ta) => {
            let (elem_sql, nullable) = map_type_to_sql(&ta.elem);
            (format!("{}[]", elem_sql), nullable)
        }
        _ => ("text".to_string(), false),
    }
}
