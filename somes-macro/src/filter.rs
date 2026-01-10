use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, PathArguments, Type};

pub(crate) struct RebuildInfo {
    pub(crate) updated_path: TokenStream,
    has_updated: bool,
    pub(crate) unrecognized_ident: Option<Ident>,
}

pub(crate) fn rebuild_type(ty: &Type, convert_to_vec: bool) -> RebuildInfo {
    let mut rebuilt_info = rebuild_type_inner(ty, convert_to_vec);
    if !rebuilt_info.has_updated && rebuilt_info.unrecognized_ident.is_none() {
        rebuilt_info.updated_path = if convert_to_vec {
            quote! { FilterOp<Vec<#ty>> }
        } else {
            quote! { FilterOp<#ty> }
        };
    }
    rebuilt_info
}

fn rebuild_type_inner(ty: &Type, convert_to_vec: bool) -> RebuildInfo {
    match ty {
        Type::Path(tp) => {
            if let Some(seg) = tp.path.segments.last() {
                let ident = seg.ident.to_string();
                match ident.as_str() {
                    "Option" | "Vec" => {
                        // extract inner generic
                        if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                                let rebuild_info = rebuild_type_inner(inner_ty, convert_to_vec);
                                let inner = rebuild_info.updated_path;
                                let ident = &seg.ident;
                                let new_path = if rebuild_info.unrecognized_ident.is_none()
                                    && !rebuild_info.has_updated
                                {
                                    if convert_to_vec {
                                        quote::quote! {
                                            #ident<FilterOp<Vec<#inner>>>
                                        }
                                    } else {
                                        quote::quote! {
                                            #ident<FilterOp<#inner>>
                                        }
                                    }
                                } else {
                                    quote::quote! {
                                        #ident<#inner>
                                    }
                                };

                                return RebuildInfo {
                                    updated_path: new_path,
                                    has_updated: rebuild_info.unrecognized_ident.is_none()
                                        || rebuild_info.has_updated,
                                    unrecognized_ident: rebuild_info.unrecognized_ident,
                                };
                            }
                        }
                    }
                    _ => {}
                }
            }

            let last = tp.path.segments.last().unwrap();
            let ident = last.ident.to_string();

            let unrecognized_ident = match ident.as_str() {
                "i8" | "i16" | "i32" | "isize" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize"
                | "f32" | "f64" | "bool" | "String" | "str" | "Uuid" | "Json" | "Value"
                | "NaiveDate" | "NaiveDateTime" | "DateTime" | "BigDecimal" => None,

                _ => Some(last.ident.clone()),
            };

            return RebuildInfo {
                updated_path: quote! { #last },
                has_updated: ident == "FilterOp",
                unrecognized_ident,
            };
        }
        // Type::Reference(tr) => rebuild_type(&tr.elem),
        // Type::Array(ta) => {
        //     let (elem_sql, nullable) = rebuild_type(&ta.elem);
        //     todo!()
        //     // (format!("{}[]", elem_sql), nullable)
        // }
        _ => todo!(),
    }
}
