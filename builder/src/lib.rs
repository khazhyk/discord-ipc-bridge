
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::PathParameters::AngleBracketed;
use syn::PathSegment;
use syn::AngleBracketedParameterData;
use quote::Tokens;

#[proc_macro_derive(DefaultBuilder)]
pub fn cool_builder(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_default_builder(&ast);
    // panic!("{}", gen.to_string());
    gen.parse().unwrap()
}

fn type_to_turbofish(mut path: syn::Path) -> syn::Path {
    let mut split = false;

    if let &AngleBracketed(ref params) = &path.segments[0].parameters {
        if params.types.len() != 0 {
            split = true;
        }
    }

    if split {
        let first = path.segments.remove(0);

        path.segments.insert(
            0,
            PathSegment {
                ident: syn::Ident::from(""),
                parameters: first.parameters.clone(), // fun
            },
        );
        path.segments.insert(
            0,
            PathSegment {
                ident: first.ident,
                parameters: AngleBracketed(AngleBracketedParameterData::default()),
            },
        );
    }

    path
}

fn impl_default_builder(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let builder_name = syn::Ident::from(name.to_string() + "Builder");

    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {

        let mut impl_tokens = Tokens::new();
        let mut build_tokens = Tokens::new();
        let mut struct_tokens = Tokens::new();

        for field in fields {
            let fty = match &field.ty {
                &syn::Ty::Path(ref o, ref p) => {
                    syn::Ty::Path(o.clone(), type_to_turbofish(p.clone()))
                }
                _ => field.ty.clone(), //whyyy
            };
            let fname = &field.ident;

            struct_tokens.append(quote! {
                #fname: Option<#fty>,
            });
            build_tokens.append(quote! {
                #fname: match self.#fname {
                    Some(b_val) => {
                        b_val
                    },
                    None => {
                        #fty::default()
                    }
                },
            });
            impl_tokens.append(quote! {
                pub fn #fname<_VAL: Into<#fty>>(mut self, val: _VAL) -> Self {
                    self.#fname = Some(val.into());
                    self
                }
            });
        }

        quote! {
            #[derive(Default)]
            pub struct #builder_name {
                #struct_tokens
            }
            #[allow(dead_code)]
            impl #builder_name {
                #impl_tokens

                pub fn build(self) -> #name {
                    #name {
                        #build_tokens
                    }
                }
            }
            impl Into<#name> for #builder_name {
                fn into(self) -> #name {
                    self.build()
                }
            }
            impl Into<Option<#name>> for #builder_name {
                fn into(self) -> Option<#name> {
                    Some(self.build())
                }
            }
            impl #name {
                #[allow(dead_code)]
                pub fn builder() -> #builder_name {
                    #builder_name::default()
                }
            }
        }
    } else {
        panic!("struct pls")
    }
}
