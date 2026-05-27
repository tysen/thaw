use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

/// Derives a string-valued method on a fieldless enum.
///
/// With `#[thaw(class = "thaw-foo")]`, generates `theme_class(&self) -> &'static str`
/// returning the BEM modifier class `"thaw-foo--<kebab-variant>"`, concatenated at
/// compile time so there is no per-call allocation.
///
/// Without the attribute, generates `as_str(&self) -> &'static str` returning the
/// kebab-cased variant name (used for HTML attribute / CSS values, or for the rare
/// enum rendered under more than one class prefix).
#[proc_macro_derive(ThemeClass, attributes(thaw))]
pub fn theme_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let Data::Enum(data) = &input.data else {
        panic!("ThemeClass can only be derived for enums");
    };

    let mut class_prefix = None;
    for attr in &input.attrs {
        if attr.path().is_ident("thaw") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("class") {
                    let value: LitStr = meta.value()?.parse()?;
                    class_prefix = Some(value.value());
                    Ok(())
                } else {
                    Err(meta.error("unknown `thaw` attribute, expected `class`"))
                }
            })
            .expect("invalid `#[thaw(...)]` attribute");
        }
    }

    let variants: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            assert!(
                matches!(variant.fields, Fields::Unit),
                "ThemeClass only supports unit (fieldless) variants"
            );
            &variant.ident
        })
        .collect();

    let (method, strings) = if let Some(prefix) = &class_prefix {
        let strings = variants
            .iter()
            .map(|ident| format!("{prefix}--{}", ident.to_string().to_kebab_case()))
            .collect::<Vec<_>>();
        (quote::format_ident!("theme_class"), strings)
    } else {
        let strings = variants
            .iter()
            .map(|ident| ident.to_string().to_kebab_case())
            .collect::<Vec<_>>();
        (quote::format_ident!("as_str"), strings)
    };

    quote! {
        impl #enum_name {
            pub fn #method(&self) -> &'static str {
                match self {
                    #(Self::#variants => #strings,)*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(WriteCSSVars)]
pub fn write_css_vars(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("Expected a struct!"),
    };

    let mut css_var_names = vec![];
    let mut field_names = vec![];
    match data.fields {
        Fields::Named(fields) => {
            for field in fields.named {
                let field_name = field.ident.unwrap();
                css_var_names.push(format!(
                    "--{}: {{}};",
                    field_name.to_string().to_lower_camel_case()
                ));
                field_names.push(field_name);
            }
        }
        _ => panic!("Expected named fields!"),
    };

    let field_names = field_names.iter();
    quote! {
        impl #struct_name {
            pub fn write_css_vars(&self, css_vars: &mut String) {
                #(css_vars.push_str(&format!(#css_var_names, self.#field_names));)*
            }
        }
    }
    .into()
}
