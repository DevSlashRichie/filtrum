use std::collections::HashMap;

use darling::{ast, util, FromDeriveInput, FromField};
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Type};

enum FilterType<'a> {
    Number(&'a Ident, Option<String>),
    String(&'a Ident, Option<String>),
    None(&'a Ident, Option<String>),
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(filtrum), supports(struct_named))]
struct MacroArgs {
    data: ast::Data<util::Ignored, FieldMacroArgs>,
    #[darling(default)]
    table: Option<String>,
}

#[derive(Debug, FromField)]
#[darling(attributes(filtrum))]
struct FieldMacroArgs {
    ident: Option<Ident>,
    ty: Type,

    #[darling(default)]
    table: Option<String>,
    #[darling(default)]
    alias: Option<String>,
    #[darling(default)]
    skip: bool,
}

fn expand_from_query_filter(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let data = MacroArgs::from_derive_input(&input)?;

    let custom_table = data
        .data
        .as_ref()
        .map_struct_fields(|x| {
            if x.table.is_some() && x.ident.is_some() {
                Some((
                    x.ident.clone().unwrap().to_string(),
                    x.table.clone().unwrap(),
                ))
            } else {
                None
            }
        })
        .take_struct()
        .unwrap()
        .into_iter()
        .filter_map(|x| x)
        .collect::<HashMap<String, String>>();

    let skipped_fields = data
        .data
        .as_ref()
        .map_struct_fields(|x| {
            if x.skip {
                Some(x.ident.clone().unwrap())
            } else {
                None
            }
        })
        .take_struct()
        .unwrap()
        .into_iter()
        .filter_map(|x| x)
        .map(|x| {
            quote! {
                #x: Default::default()
            }
        })
        .collect::<Vec<_>>();

    let fields = data
        .data
        .as_ref()
        .map_struct_fields(|x| {
            if x.skip {
                return None;
            }

            Some(x)
        })
        .take_struct()
        .unwrap()
        .iter()
        .filter_map(|x| if let Some(x) = x { Some(x) } else { None })
        .filter_map(|f| {
            let name = &f.ident;
            let ty = &f.ty;
            let alias = &f.alias;

            if name.is_none() && alias.is_none() {
                return None;
            }

            let alias = alias.as_ref().map(|x| x.to_string());

            // if type is Option<NumberFilter<T>> then
            // return (name, ty)

            if let Type::Path(type_path) = ty {
                if let Some(segment) = type_path.path.segments.last() {
                    let ident = &segment.ident;
                    if ident == "NumberFilters" {
                        return Some(FilterType::Number(name.as_ref().unwrap(), alias));
                    }
                    if ident == "StringFilters" {
                        return Some(FilterType::String(name.as_ref().unwrap(), alias));
                    }

                    return Some(FilterType::None(name.as_ref().unwrap(), alias));
                }
            }

            None
        })
        .collect::<Vec<_>>();

    fn create_search_id(
        table: &str,
        ident: &Ident,
        alias: &Option<String>,
        place: impl FnOnce() -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let var_name = format_ident!("{}", ident);
        let ran = place();

        match alias {
            Some(alias) => quote! {
                let search_id = filtrum::FilterId::WithPrefixAndAlias(#table.to_string(), stringify!(#var_name).to_string(), #alias.to_string());
                #ran
            },
            None => quote! {
                let search_id = filtrum::FilterId::WithPrefix(#table.to_string(), stringify!(#var_name).to_string());
                #ran
            },
        }
    }

    let fields_as_filters = fields.iter().filter_map(|f| {
            let f = match f {
                FilterType::Number(ident, alias) => {
                    let var_name = format_ident!("{}", ident);

                    if let Some(table) = custom_table.get(&ident.to_string()) {
                        create_search_id(table, ident, alias, || quote! {
                            let #var_name = filtrum::NumberFilters::from_id_value(search_id, s)?;
                        })
                    } else {
                        quote! {
                            let #var_name = filtrum::NumberFilters::from_str(stringify!(#var_name), s)?;
                        }
                    }

                }
                FilterType::String(ident, alias) => {
                    let var_name = format_ident!("{}", ident);

                    if let Some(table) = custom_table.get(&ident.to_string()) {
                        create_search_id(table, ident, alias, || quote! {
                            let #var_name = filtrum::StringFilters::from_id_value(search_id, s)?;
                        })
                    } else {
                        quote! {
                            let #var_name = filtrum::StringFilters::from_str(stringify!(#var_name), s)?;
                        }
                    }
                }
                FilterType::None(ident, alias) => {
                    let var_name = format_ident!("{}", ident);

                    if let Some(table) = custom_table.get(&ident.to_string()) {
                        create_search_id(table, ident, alias, || quote! {
                            let #var_name = filtrum::EqualFilter::from_id_value(search_id, s)?;
                        })
                    } else {
                        quote! {
                            let #var_name = filtrum::EqualFilter::from_str(stringify!(#var_name), s)?;
                        }
                    }
                }
            };

            Some(f)
        })
        .collect::<Vec<_>>();

    let impl_with_filter_id = if let Some(table) = data.table {
        quote! {
            Some(#table)
        }
    } else {
        quote! { None }
    };

    let field_names = fields
        .iter()
        .filter_map(|f| {
            let r = match f {
                FilterType::Number(f, _) => {
                    quote! {
                        #f
                    }
                }
                FilterType::String(f, _) => {
                    quote! {
                        #f
                    }
                }
                FilterType::None(f, _) => {
                    quote! {
                        #f
                    }
                }
            };

            Some(r)
        })
        .collect::<Vec<_>>();

    let impl_into_cond = quote! {
        impl filtrum::WithFilterId for #name {
            fn filter_id() -> Option<&'static str> {
                #impl_with_filter_id
            }
        }

    };

    let all_fields = skipped_fields.iter().chain(field_names.iter());

    Ok(quote! {
        #[automatically_derived]
        impl std::str::FromStr for #name {
            type Err = filtrum::FilterParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #(#fields_as_filters)*
                Ok(Self {
                    #(#all_fields),*
                })
            }
        }

        #impl_into_cond
    })
}

#[proc_macro_derive(Filterable, attributes(filtrum))]
pub fn derive_filterable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let toks = expand_from_query_filter(&input).unwrap_or_else(|e| e.to_compile_error());

    toks.into()
}
