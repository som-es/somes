use heck::ToSnakeCase;
use syn::{GenericArgument, PathArguments, Type};

pub(crate) fn map_type_to_sql(ty: &Type) -> (String, bool) {
    match ty {
        Type::Path(tp) => {
            if let Some(seg) = tp.path.segments.last() {
                let ident = seg.ident.to_string();
                match ident.as_str() {
                    "Option" => {
                        // extract inner generic
                        if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                                let (s, _inner_nullable) = map_type_to_sql(inner_ty);
                                return (s, true);
                            }
                        }
                    }
                    "Vec" => {}
                    _ => {}
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
                "Vec" => {
                    if let PathArguments::AngleBracketed(inner_generics) = &last.arguments {
                        let GenericArgument::Type(ty) = inner_generics.args.first().unwrap() else {
                            panic!("Vec should only take a single generic type (typically)")
                        };
                        let (elem_sql, nullable) = map_type_to_sql(ty);
                        return (format!("{}[]", elem_sql), nullable);
                    } else {
                        panic!("Vec contains nothing?")
                    }
                }

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
