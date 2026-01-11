use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericArgument, Ident, PathArguments, Type};

pub(crate) struct RebuildInfo {
    pub(crate) updated_path: TokenStream,
    has_updated: bool,
    pub(crate) unrecognized_ident: Option<Ident>,
}

pub(crate) fn rebuild_type(
    ty: &Type,
    convert_to_vec: bool,
    convert_to_optional: bool,
) -> RebuildInfo {
    let mut rebuilt_info = rebuild_type_inner(ty, convert_to_vec, convert_to_optional);
    if !rebuilt_info.has_updated && rebuilt_info.unrecognized_ident.is_none() {
        let mut updated_path = if convert_to_vec {
            quote! { FilterOp<Vec<#ty>> }
        } else {
            quote! { FilterOp<#ty> }
        };
        if convert_to_optional {
            updated_path = quote! { Option<#updated_path> };
        }
        rebuilt_info.updated_path = updated_path;
    }
    rebuilt_info
}

fn rebuild_type_inner(ty: &Type, convert_to_vec: bool, convert_to_optional: bool) -> RebuildInfo {
    match ty {
        Type::Path(tp) => {
            if let Some(seg) = tp.path.segments.last() {
                let ident = seg.ident.to_string();
                match ident.as_str() {
                    "Option" | "Vec" | "Json" => {
                        // extract inner generic
                        if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                                let rebuild_info = rebuild_type_inner(
                                    inner_ty,
                                    convert_to_vec,
                                    convert_to_optional,
                                );
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
                                        let is_vec = ident == "Vec";
                                        if is_vec {
                                            quote::quote! {
                                                FilterOp<#ident<#inner>>
                                            }
                                        } else {
                                            quote::quote! {
                                                #ident<FilterOp<#inner>>
                                            }
                                        }
                                    }
                                } else {
                                    if ident == "Json" {
                                        quote! {
                                            #inner
                                        }
                                    } else {
                                        quote::quote! {
                                            #ident<#inner>
                                        }
                                    }
                                };
                                let new_path = if convert_to_optional {
                                    if ident == "Option" {
                                        new_path
                                    } else {
                                        quote! { Option<#new_path> }
                                    }
                                } else {
                                    new_path
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

            let (updated_path, unrecognized_ident) = match ident.as_str() {
                "i8" | "i16" | "i32" | "isize" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize"
                | "f32" | "f64" | "bool" | "String" | "str" | "Uuid" | "Value" | "NaiveDate"
                | "NaiveDateTime" | "DateTime" | "BigDecimal" => (quote! { #last }, None),

                _ => {
                    let ident = format_ident!("{}Filter", last.ident.clone());
                    (quote! { #ident }, Some(last.ident.clone()))
                }
            };

            return RebuildInfo {
                updated_path,
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
