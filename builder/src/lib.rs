use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Error, Ident, Result, Type};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_builder(input) {
        Ok(token) => TokenStream::from(token),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

fn derive_builder(input: DeriveInput) -> Result<TokenStream2> {
    use syn::{Data, DataStruct, Fields, FieldsNamed};
    if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = input.data
    {
        let (input_name, vis) = (input.ident, input.vis);
        let builder_name = format_ident!("{}Builder", input_name);
        let fields = named
            .iter()
            .map(|f| (f.ident.as_ref().expect("field name not found"), &f.ty));
        let idents = fields.clone().map(|(ident, _)| ident);
        let builder_fields = fields
            .clone()
            .map(|(i, t)| quote! {#i: ::core::option::Option<#t>});
        let new_builder = fields.clone().map(__new);

        let mut each_names = Vec::with_capacity(named.len());
        for field in named.iter() {
            if let Some(attr) = field.attrs.first() {
                each_names.push(each(attr)?);
            } else {
                each_names.push(None);
            }
        }

        let (more, impl_fns): (Vec<_>, Vec<_>) = fields
            .clone()
            .zip(each_names)
            .map(|((ident, ty), each_name)| match each_name {
                Some(name) => (&name != ident, impl_fn(&vis, ident, ty, Some(&name))),
                None => (false, impl_fn(&vis, ident, ty, None)),
            })
            .unzip();
        #[rustfmt::skip]
        let impl_fns_more = fields.zip(more)
            .filter_map(|((ident, ty), m)| { if m { Some(impl_fn(&vis, ident, ty, None)) } else { None } });

        Ok(quote! {
            #vis struct #builder_name {
                #(#builder_fields),*
            }

            impl #builder_name {
                #(#impl_fns)*
                #(#impl_fns_more)*

                #vis fn build(&mut self) ->
                    ::core::result::Result<#input_name, std::boxed::Box<dyn ::std::error::Error>>
                {
                    Ok(#input_name {
                        #(
                            #idents : self.#idents.take().ok_or_else(||
                                format!("`{}` is not set", stringify!(#idents))
                            )?
                        ),*
                    })
                }
            }

            impl #input_name {
                #vis fn builder() -> #builder_name {
                    #builder_name {
                        #(#new_builder),*
                    }
                }
            }
        })
    } else {
        Err(Error::new(input.span(), "Named Struct Only :)"))
    }
}

fn impl_fn(
    vis: &syn::Visibility,
    ident: &Ident,
    mut ty: &Type,
    each_name: Option<&Ident>,
) -> TokenStream2 {
    let vec_t = each_name.is_some();
    match check(&mut ty, vec_t) {
        CheckFieldType::Option => quote! {
            #vis fn #ident (&mut self, #ident : #ty) -> &mut Self {
                self.#ident = ::core::option::Option::Some(::core::option::Option::Some(#ident));
                self
            }
        },
        CheckFieldType::Vec if vec_t => {
            let each_name = each_name.expect("failed to get `each` name");
            quote! {
                #vis fn #each_name (&mut self, #each_name : #ty) -> &mut Self {
                    self.#ident.as_mut().map(|v| v.push(#each_name));
                    self
                }
            }
        }
        _ => quote! {
            #vis fn #ident (&mut self, #ident : #ty) -> &mut Self {
                self.#ident = ::core::option::Option::Some(#ident);
                self
            }
        },
    }
}

fn __new((ident, mut ty): (&Ident, &Type)) -> TokenStream2 {
    match check(&mut ty, false) {
        CheckFieldType::Option => {
            quote! {#ident: ::core::option::Option::Some(::core::option::Option::None)}
        }
        CheckFieldType::Vec => {
            quote! {#ident: ::core::option::Option::Some(::std::vec::Vec::new())}
        }
        _ => quote! {#ident: ::core::option::Option::None},
    }
}

// 把 Option<T> 转化成 T
fn check(ty: &mut &Type, vec_t: bool) -> CheckFieldType {
    use syn::{
        AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, TypePath,
    };
    if let Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon,
            segments,
        },
    }) = ty
    {
        if leading_colon.is_none() && segments.len() == 1 {
            if let Some(PathSegment {
                ident,
                arguments:
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
            }) = segments.first()
            {
                if let (1, Some(GenericArgument::Type(t))) = (args.len(), args.first()) {
                    if ident == "Option" {
                        *ty = t;
                        return CheckFieldType::Option;
                    } else if ident == "Vec" {
                        if vec_t {
                            *ty = t;
                        }
                        return CheckFieldType::Vec;
                    }
                }
            }
        }
    }
    CheckFieldType::Raw
}

enum CheckFieldType {
    Raw,
    Option,
    Vec,
}

fn each(attr: &syn::Attribute) -> Result<Option<Ident>> {
    use syn::{LitStr, Meta};
    let mut id = Ok(None);
    let meta_ = &attr.meta;
    if let Meta::List(l) = meta_ {
        if l.path.is_ident("builder") {
            l.parse_nested_meta(|meta| {
                if meta.path.is_ident("each") {
                    let s: LitStr = meta.value()?.parse()?;
                    id = Ok(Some(format_ident!("{}", s.value())));
                } else {
                    id = Err(Error::new(
                        meta_.span(),
                        "expected `builder(each = \"...\")`",
                    ));
                }
                Ok(())
            })
            .unwrap_or_else(|_| {
                id = Err(Error::new(
                    meta_.span(),
                    "expected `builder(each = \"...\")`",
                ))
            })
        }
    }
    id
}
