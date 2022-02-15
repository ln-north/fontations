use syn::spanned::Spanned;

use super::{ArrayField, ScalarField, ScalarType};

/// All of the attrs that can be applied to a field.
///
/// These are not validated, and do not all make sense on all fields;
/// rather they are just collected here.
#[derive(Default)]
pub struct FieldAttrs {
    docs: Vec<syn::Attribute>,
    hidden: Option<syn::Path>,
    count: Option<Count>,
    variable_size: Option<syn::Path>,
}

/// Annotations for how to calculate the count of an array.
pub enum Count {
    Field(syn::Ident),
    Literal(syn::LitInt),
    All(syn::Path),
    Function {
        fn_: syn::Path,
        args: Vec<syn::Ident>,
    },
}

#[derive(Default)]
pub struct VariantAttrs {
    pub docs: Vec<syn::Attribute>,
    pub version: Option<syn::Path>,
}

#[derive(Default)]
pub struct ItemAttrs {
    pub docs: Vec<syn::Attribute>,
    pub format: Option<syn::Ident>,
}

impl FieldAttrs {
    pub fn parse(attrs: &[syn::Attribute]) -> Result<FieldAttrs, syn::Error> {
        let mut result = FieldAttrs::default();
        for attr in attrs {
            match attr.parse_meta()? {
                syn::Meta::NameValue(value) if value.path.is_ident("doc") => {
                    result.docs.push(attr.clone());
                }
                syn::Meta::Path(path) if path.is_ident("hidden") => {
                    result.hidden = Some(path.clone())
                }
                syn::Meta::Path(path) if path.is_ident("variable_size") => {
                    result.variable_size = Some(path.clone())
                }
                syn::Meta::Path(path) if path.is_ident("count_all") => {
                    result.count = Some(Count::All(path.clone()));
                }

                syn::Meta::List(list) if list.path.is_ident("count") => {
                    let inner = expect_single_item_list(&list)?;
                    match inner {
                        syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.get_ident().is_some() => {
                            result.count = Some(Count::Field(p.get_ident().unwrap().clone()));
                        }
                        syn::NestedMeta::Lit(syn::Lit::Int(int)) => {
                            result.count = Some(Count::Literal(int));
                        }
                        _ => return Err(syn::Error::new(
                            list.path.span(),
                            "count attribute should have format #[count(field)] or #[count(123)]",
                        )),
                    }
                }
                syn::Meta::List(list) if list.path.is_ident("count_with") => {
                    let mut items = list.nested.iter();
                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = items.next() {
                        let args = items.map(expect_ident).collect::<Result<_, _>>()?;
                        assert!(result.count.is_none(), "I ONLY COUNT ONCE");
                        result.count = Some(Count::Function {
                            fn_: path.to_owned(),
                            args,
                        });
                        continue;
                    }
                    return Err(syn::Error::new(
                        list.path.span(),
                        "count_with attribute should have format count_with(path::to::fn, arg1, arg2)",
                    ));
                }
                other => {
                    return Err(syn::Error::new(other.span(), "unknown attribute"));
                }
            }
        }
        Ok(result)
    }

    pub fn into_array(
        self,
        name: syn::Ident,
        inner_typ: syn::Ident,
        inner_lifetime: Option<syn::Lifetime>,
    ) -> Result<ArrayField, syn::Error> {
        if let Some(path) = &self.hidden {
            return Err(syn::Error::new(
                path.span(),
                "'hidden' is only valid on scalar fields",
            ));
        }
        let count = self.count.ok_or_else(|| {
            syn::Error::new(
                name.span(),
                "array types require 'count' or 'count_with' attribute",
            )
        })?;
        let variable_size = self.variable_size;
        Ok(ArrayField {
            docs: self.docs,
            name,
            inner_typ,
            inner_lifetime,
            count,
            variable_size,
        })
    }

    pub fn into_scalar(self, name: syn::Ident, typ: ScalarType) -> Result<ScalarField, syn::Error> {
        if let Some(span) = self.count.as_ref().map(Count::span) {
            return Err(syn::Error::new(
                span,
                "count/count_with attribute not valid on scalar fields",
            ));
        }
        if let Some(token) = self.variable_size {
            return Err(syn::Error::new(token.span(), "not valid on scalar fields"));
        }

        Ok(ScalarField {
            docs: self.docs,
            name,
            typ,
            hidden: self.hidden,
        })
    }
}

impl Count {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Count::All(path) => path.span(),
            Count::Field(ident) => ident.span(),
            Count::Function { fn_, .. } => fn_.span(),
            Count::Literal(lit) => lit.span(),
        }
    }
}

static VERSION: &str = "version";
impl VariantAttrs {
    pub fn parse(attrs: &[syn::Attribute]) -> Result<VariantAttrs, syn::Error> {
        let mut result = VariantAttrs::default();
        for attr in attrs {
            match attr.parse_meta()? {
                syn::Meta::NameValue(value) if value.path.is_ident("doc") => {
                    result.docs.push(attr.clone());
                }
                syn::Meta::List(list) if list.path.is_ident(VERSION) => {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(p))) = list.nested.first() {
                        result.version = Some(p.clone());
                    } else {
                        return Err(syn::Error::new(
                            list.path.span(),
                            "version attribute should have format version(path::to::CONST_VERSION)",
                        ));
                    }
                }
                other => return Err(syn::Error::new(other.span(), "unknown attribute")),
            }
        }
        Ok(result)
    }
}

static FORMAT: &str = "format";
impl ItemAttrs {
    pub fn parse(attrs: &[syn::Attribute]) -> Result<ItemAttrs, syn::Error> {
        let mut result = ItemAttrs::default();
        for attr in attrs {
            //if let syn::Attribute::break
            match attr.parse_meta()? {
                syn::Meta::NameValue(value) if value.path.is_ident("doc") => {
                    result.docs.push(attr.clone());
                }
                syn::Meta::List(list) if list.path.is_ident(FORMAT) => {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(p))) = list.nested.first() {
                        if let Some(ident) = p.get_ident() {
                            result.format = Some(ident.clone());
                            continue;
                        }
                    }

                    return Err(syn::Error::new(
                        list.path.span(),
                        "format attribute should  be in form 'version(ScalarType)'",
                    ));
                }
                other => return Err(syn::Error::new(dbg!(other).span(), "unknown attribute")),
            }
        }
        Ok(result)
    }
}

fn expect_single_item_list(meta: &syn::MetaList) -> Result<syn::NestedMeta, syn::Error> {
    match meta.nested.first() {
        Some(item) if meta.nested.len() == 1 => Ok(item.clone()),
        _ => Err(syn::Error::new(meta.span(), "expected single item list")),
    }
}

fn expect_ident(meta: &syn::NestedMeta) -> Result<syn::Ident, syn::Error> {
    match meta {
        syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.get_ident().is_some() => {
            Ok(p.get_ident().unwrap().clone())
        }
        _ => Err(syn::Error::new(meta.span(), "expected ident")),
    }
}
