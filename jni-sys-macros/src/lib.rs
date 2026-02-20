extern crate proc_macro;

use std::{cmp::Ordering, collections::HashSet};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Data, DeriveInput, Fields, Ident,
    LitStr, Token,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Version {
    major: u16,
    minor: u16,
}

impl Default for Version {
    fn default() -> Self {
        Self { major: 1, minor: 1 }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => self.minor.cmp(&other.minor),
            major_order => major_order,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<LitStr> for Version {
    type Error = syn::Error;

    fn try_from(version: LitStr) -> Result<Self, Self::Error> {
        let span = version.span();
        let version = version.value();
        if version == "reserved" {
            // We special case version 999 later instead of making JniVersion an enum
            return Ok(Version {
                major: 999,
                minor: 0,
            });
        }
        let mut split = version.splitn(2, '.');
        const EXPECTED_MSG: &str = "Expected \"major.minor\" version number or \"reserved\"";
        let major = split.next().ok_or(syn::Error::new(span, EXPECTED_MSG))?;
        let major = major
            .parse::<u16>()
            .map_err(|_| syn::Error::new(span, EXPECTED_MSG))?;
        let minor = split
            .next()
            .unwrap_or("0")
            .parse::<u16>()
            .map_err(|_| syn::Error::new(span, EXPECTED_MSG))?;
        Ok(Version { major, minor })
    }
}

impl syn::parse::Parse for Version {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<LitStr>()?.try_into()
    }
}

struct Config {
    name: String,
    version_default: Version,
}

impl syn::parse::Parse for Config {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?.into_iter();
        let name = args
            .next()
            .map(|s| s.value())
            .unwrap_or_else(|| "JNI".to_string());
        let version_default = args
            .next()
            .map(Version::try_from)
            .transpose()?
            .unwrap_or_default();
        Ok(Self {
            name,
            version_default,
        })
    }
}

fn jni_to_union_impl(cfg: Config, input: DeriveInput) -> syn::Result<TokenStream> {
    let original_name = &input.ident;
    let original_visibility = &input.vis;

    let mut versions = HashSet::new();
    let mut versioned_fields = vec![];

    let Config {
        name,
        version_default,
    } = cfg;

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                let mut min_version = version_default;

                let mut field = field.clone();

                let mut jni_added_attr = None;
                field.attrs.retain(|attr| {
                    if attr.path().is_ident("jni_added") {
                        jni_added_attr = Some(attr.clone());
                        false
                    } else {
                        true
                    }
                });
                if let Some(attr) = jni_added_attr {
                    let version = attr.parse_args::<Version>()?;
                    min_version = version;
                }

                versions.insert(min_version);
                versioned_fields.push((min_version, field.clone()));
            }

            // Quote structs and union
            let mut expanded = quote! {};

            let mut union_members = quote!();

            let mut versions: Vec<_> = versions.into_iter().collect();
            versions.sort();

            for version in versions {
                let (struct_ident, version_ident, version_suffix) = if version.major == 999 {
                    (
                        Ident::new(&format!("{}_reserved", original_name), original_name.span()),
                        Ident::new("reserved", original_name.span()),
                        "reserved".to_string(),
                    )
                } else if version.minor == 0 {
                    (
                        Ident::new(
                            &format!("{}_{}", original_name, version.major),
                            original_name.span(),
                        ),
                        Ident::new(&format!("v{}", version.major), original_name.span()),
                        format!("{}", version.major),
                    )
                } else {
                    let struct_ident = Ident::new(
                        &format!("{}_{}_{}", original_name, version.major, version.minor),
                        original_name.span(),
                    );
                    let version_ident = Ident::new(
                        &format!("v{}_{}", version.major, version.minor),
                        original_name.span(),
                    );
                    (
                        struct_ident,
                        version_ident,
                        format!("{}_{}", version.major, version.minor),
                    )
                };

                let last = versioned_fields
                    .iter()
                    .rposition(|(v, _f)| v <= &version)
                    .unwrap_or(versioned_fields.len());
                let mut padding_idx = 0u32;

                let mut version_field_tokens = quote!();
                for (i, (field_min_version, field)) in versioned_fields.iter().enumerate() {
                    if i > last {
                        break;
                    }
                    if field_min_version > &version {
                        let reserved_ident = format_ident!("_padding_{}", padding_idx);
                        padding_idx += 1;
                        version_field_tokens.extend(quote! { #reserved_ident: *mut c_void, });
                    } else {
                        version_field_tokens.extend(quote! { #field, });
                    }
                }
                expanded.extend(quote! {
                    #[allow(non_snake_case, non_camel_case_types)]
                    #[repr(C)]
                    #[derive(Copy, Clone)]
                    #original_visibility struct #struct_ident {
                        #version_field_tokens
                    }
                });

                let api_comment =
                    format!("API when {name} version >= `{name}_VERSION_{version_suffix}`");
                union_members.extend(quote! {
                    #[doc = #api_comment]
                    #original_visibility #version_ident: #struct_ident,
                });
            }

            expanded.extend(quote! {
                #[repr(C)]
                #original_visibility union #original_name {
                    #union_members
                }
            });

            return Ok(TokenStream::from(expanded));
        }
    }

    Err(syn::Error::new(
        input.span(),
        "Expected a struct with fields",
    ))
}

#[proc_macro_attribute]
pub fn jni_to_union(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(attr as Config);

    match jni_to_union_impl(args, input) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}
