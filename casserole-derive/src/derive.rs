use proc_macro2::{TokenStream as Tokens, Span, TokenTree, Delimiter};
use syn::{Data, DataEnum, DeriveInput, Fields, Ident, Variant, WhereClause, parse_quote, GenericParam};
use quote::quote;

#[derive(Copy, Clone)]
enum Sep {
    Comma,
    #[allow(unused)]
    SemiColon,
}

#[derive(Default)]
struct MainAttrs {
}

#[derive(Default)]
struct FieldAttrs {
    store: bool,
}

enum FieldsKind {
    Named,
    Unnamed,
    Unit,
}

pub(crate) fn derive_object_casserole(krate: &'static str, input: &DeriveInput) -> Tokens {
    let krate = syn::Ident::new(krate, Span::call_site());

    let name = &input.ident;
    let s = syn::GenericParam::Type(parse_quote! {S});
    let mut generics = input.generics.clone();

    let mut attrs = MainAttrs::default();
    for attribute in &input.attrs {
        get_main_attr_info(attribute, &mut attrs);
    }

    let mut folded_type = input.clone();
    let folded_type_ident = syn::Ident::new(
        &format!("{}_Casserole", input.ident),
        Span::call_site(),
    );

    folded_type.ident = folded_type_ident.clone();
    folded_type.attrs.clear();
    folded_type.attrs.push(parse_quote!{#[derive(Serialize, Deserialize)]});
    folded_type.attrs.push(parse_quote!{#[allow(non_camel_case_types)]});
    folded_type.attrs.push(parse_quote!{#[serde(bound = "S::Key: DeserializeOwned")]});
    folded_type.vis = parse_quote!{pub};

    let s_where : WhereClause = parse_quote!{where #s: #krate::store::Store};
    folded_type.generics.where_clause = Some({
        let mut all_where = s_where.clone();
        for param in &generics.params {
            if let GenericParam::Type(ref type_param) = *param {
                let ident = &type_param.ident;
                all_where.predicates.push(parse_quote!{#ident: Casserole<#s>});
                all_where.predicates.push(parse_quote!{<#ident as Casserole<#s>>::Target: DeserializeOwned});
            }
        }
        all_where
    });

    folded_type.generics.params.insert(0, s.clone());
    let target_type_generics = folded_type.generics.params.clone();

    if let Some(where_clause) = &mut generics.where_clause {
        where_clause.predicates.extend(s_where.predicates.into_iter().collect::<Vec<_>>());
    } else {
        generics.where_clause = Some(s_where.clone());
    }
    if let Some(where_clause) = &mut generics.where_clause {
        for param in &generics.params {
            if let GenericParam::Type(ref type_param) = *param {
                let ident = &type_param.ident;
                where_clause.predicates.push(parse_quote!{#ident: Casserole<#s>});
                where_clause.predicates.push(parse_quote!{<#ident as Casserole<#s>>::Target: DeserializeOwned});
            }
        }
    }

    let handle_field = &|field: &mut syn::Field| {
        let mut attrs = FieldAttrs::default();
        for attribute in field.attrs.iter() {
            get_field_attr_info(&attribute, &mut attrs);
        }

        let ty = &field.ty;
        if attrs.store {
            field.ty = parse_quote! {
                #s::Key
            }
        } else {
            field.ty = parse_quote! {
                <#ty as Casserole<#s>>::Target
            }
        }

        field.attrs.clear();
    };

    match &mut folded_type.data {
        Data::Struct(args) => {
            for field in args.fields.iter_mut() {
                handle_field(field);
            }
        },
        Data::Enum(args) => {
            for variant in args.variants.iter_mut() {
                for field in variant.fields.iter_mut() {
                    handle_field(field);
                }
            }
        },
        Data::Union(_) => todo!(),
    }

    let mut modified_generics = generics.clone();
    modified_generics.params.insert(0, s.clone());
    let (modified_impl_generics, _, _) = modified_generics.split_for_impl();
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let casserole_gen = generator("self", Sep::Comma, input,
        |field_ext| {
            let mut attrs = FieldAttrs::default();
            for attribute in field_ext.attrs() {
                get_field_attr_info(attribute, &mut attrs);
            }
            let access_expr = field_ext.access_expr_ref();
            let match_ident = field_ext.get_match_ident();

            let casserole = if attrs.store {
                quote! { { let v = Casserole::<S>::casserole(#access_expr, store)?; store.put(&v)?  } }
            } else {
                quote! { Casserole::<S>::casserole(#access_expr, store)?  }
            };

            if field_ext.named {
                quote! { #match_ident : #casserole }
            } else {
                quote! { #casserole }
            }
        },
        |expr| quote! { Ok(#expr) },
        |expr, variant, field_kind| variant_reconstruction(quote!{Self::Target}, field_kind, variant, expr)
    );

    let input_decasserole = {
        let mut input = input.clone();
        input.ident = folded_type_ident.clone();
        input
    };

    let decasserole_gen = generator("target", Sep::Comma, &input_decasserole,
        |field_ext| {
            let mut attrs = FieldAttrs::default();
            for attribute in field_ext.attrs() {
                get_field_attr_info(attribute, &mut attrs);
            }
            let access_expr = field_ext.access_expr_ref();
            let match_ident = field_ext.get_match_ident();

            let casserole = if attrs.store {
                quote! { { let v = store.get(#access_expr)?.expect("missing key"); Casserole::<S>::decasserole(&v, store)?  } }
            } else {
                quote! { Casserole::<S>::decasserole(#access_expr, store)? }
            };

            if field_ext.named {
                quote! { #match_ident : #casserole }
            } else {
                quote! { #casserole }
            }
        },
        |expr| quote! { Ok(#expr) },
        |expr, variant, field_kind| variant_reconstruction(quote!{Self}, field_kind, variant, expr)
    );

    let result = quote! {
        #folded_type

        impl #modified_impl_generics Casserole<#s> for #name #ty_generics #where_clause {
            type Target = #folded_type_ident<#target_type_generics>;

            fn casserole(&self, store: &mut S)
                -> Result<Self::Target, S::Error>
            {
                #casserole_gen
            }

            fn decasserole(target: &Self::Target, store: &mut S)
                -> Result<Self, S::Error>
            {
                #decasserole_gen
            }
        }
    };

    let upper_name = name.to_string().to_uppercase();
    let dummy_mod = syn::Ident::new(
        &format!("_IMPL_DIESEL_NEW_TYPE_FOR_{}", upper_name),
        Span::call_site(),
    );

    let bring_type = if krate == "crate" {
        quote! { }
    } else {
        quote! { use super::#name; }
    };

    quote! {
        mod #dummy_mod {
            use super::*;

            #bring_type
            use #krate::Casserole;
            use serde::{de::DeserializeOwned, Deserialize, Serialize};

            #result
        }

        pub use #dummy_mod::#folded_type_ident;
    }
}

fn get_attr_token_expr(ts: proc_macro2::TokenStream) -> (String, proc_macro2::token_stream::IntoIter) {
    let mut ts = ts.clone().into_iter();
    let term = 'x: loop {
        match ts.next() {
            Some(TokenTree::Ident(term)) => {
                break 'x term.to_string();
            }
            Some(TokenTree::Literal(term)) => {
                let s = term.to_string();
                let p : proc_macro::TokenStream =
                    std::str::FromStr::from_str(&format!("({})", &s[1..s.len()-1])).unwrap();
                let t : TokenTree = syn::parse(p).unwrap();
                let mut inner_ts = proc_macro2::TokenStream::from(t).into_iter();
                if let Some(TokenTree::Group(group)) = inner_ts.next() {
                    ts = group.stream().into_iter();
                } else {
                    panic!("expected attribute value x: {:?}", term)
                }
            }
            term => {
                panic!("expected attribute value x: {:?}", term)
            }
        }
    };

    (term, ts)
}

fn get_main_attr_info(
    attribute: &syn::Attribute,
    #[allow(unused)]
    attrs: &mut MainAttrs,
) {
    match attribute.path.segments.last() {
        None => {
            // Ignore this attribute
            return;
        }
        Some(first_segment) => {
            if first_segment.ident.to_string() != "casserole" {
                // Ignore this attribute, it does not relate to Interact
                return;
            }
        }
    }

    for func in attribute.tokens.clone().into_iter() {
        let ts = match &func {
            TokenTree::Literal { .. } | TokenTree::Punct { .. } => {
                // Skip comments
                continue;
            }
            TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                group.stream()
            }
            _ => {
                panic!("extend () in `casserole` data type attribute `{:?}`", func);
            }
        };

        let (term, _) = get_attr_token_expr(ts);
        match term.as_str() {
            _ => panic!(
                "Invalid term {} in `{}`",
                term.as_str(),
                attribute.tokens.to_string()
            ),
        }
    }
}

fn get_field_attr_info(
    attribute: &syn::Attribute,
    attrs: &mut FieldAttrs,
) {
    match attribute.path.segments.last() {
        None => {
            // Ignore this attribute
            return;
        }
        Some(first_segment) => {
            if first_segment.ident.to_string() != "casserole" {
                // Ignore this attribute, it does not relate to Interact
                return;
            }
        }
    }

    for func in attribute.tokens.clone().into_iter() {
        let ts = match &func {
            TokenTree::Literal { .. } | TokenTree::Punct { .. } => {
                // Skip comments
                continue;
            }
            TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                group.stream()
            }
            _ => {
                panic!("extend () in `casserole` data type attribute `{:?}`", func);
            }
        };

        let (term, _) = get_attr_token_expr(ts);

        match term.as_str() {
            "store" => attrs.store = true,
            _ => panic!(
                "Invalid term {} in `{}`",
                term.as_str(),
                attribute.tokens.to_string()
            ),
        }
    }
}

fn per_struct_field<F, W>(varname: &'static str, from_enum: bool, sep: Sep, fields: &Fields, f: F, w: W) -> Tokens
where
    F: for <'a> Fn(&'a FieldExt) -> Tokens + Copy,
    W: for <'a> Fn(Tokens, &'a Vec<FieldExt<'a>>, bool) -> Tokens,
{
    match fields {
        Fields::Named(ref fields) => {
            let fields : Vec<_> = fields.named.iter().collect();

            generator_for_struct_kind(varname, from_enum, sep, Some(&fields[..]), true, f, w)
        },
        Fields::Unnamed(ref fields) => {
            let fields : Vec<_> = fields.unnamed.iter().collect();

            generator_for_struct_kind(varname, from_enum, sep, Some(&fields[..]), false, f, w)
        },
        Fields::Unit => {
            generator_for_struct_kind(varname, from_enum, sep, None, false, f, w)
        },
    }
}

fn variant_reconstruction(typeref: Tokens, field_kind: FieldsKind, variant: Option<&Variant>, expr: Tokens) -> Tokens {
    match field_kind {
        FieldsKind::Named => {
            match variant {
                None => quote!{ #typeref { #expr } },
                Some(variant) => {
                    let ident = &variant.ident;
                    quote!{ #typeref::#ident { #expr } }
                },
            }
        },
        FieldsKind::Unnamed => {
            match variant {
                None => quote!{ #typeref ( #expr ) },
                Some(variant) => {
                    let ident = &variant.ident;
                    quote!{ #typeref::#ident ( #expr ) }
                },
            }
        },
        FieldsKind::Unit => {
            match variant {
                None => quote!{ #typeref },
                Some(variant) => {
                    let ident = &variant.ident;
                    quote!{ #typeref::#ident }
                },
            }
        },
    }
}

fn generator_for_struct_kind<F, W>(varname: &'static str, from_enum: bool, sep: Sep, fields: Option<&[&syn::Field]>, named: bool, f: F, w: W) -> Tokens
where
    F: for <'a> Fn(&'a FieldExt) -> Tokens + Copy,
    W: for <'a> Fn(Tokens, &'a Vec<FieldExt<'a>>, bool) -> Tokens,
{
    let fields: Vec<_> = fields.unwrap_or(&[]).iter()
        .enumerate().map(|(i, f)| FieldExt::new(varname, from_enum, f, i, named)).collect();

    let mut i = 0;
    let ifields = fields.iter().map(|field| {
        let r = f(field);
        i += 1;
        r
    });

    w(match sep {
        Sep::Comma => quote!{ #(#ifields),* },
        Sep::SemiColon => quote!{ #(#ifields);* },
    }, &fields, named)
}

fn generator<F, W, V>(varname: &'static str, sep: Sep, input: &DeriveInput, f: F, w: W, v: V) -> Tokens
where
    F: for <'a> Fn(&'a FieldExt) -> Tokens + Copy,
    W: for <'a> Fn(Tokens) -> Tokens,
    V: for <'a> Fn(Tokens, Option<&'a Variant>, FieldsKind) -> Tokens,
{
    let name = &input.ident;

    match &input.data {
        Data::Enum(data_enum) => generator_for_enum(varname, sep, name, data_enum, f, w, v),
        Data::Struct(variant_data) => per_struct_field(varname, false, sep, &variant_data.fields,
            f, |expr, fields, named| w(v(expr, None, match (named, fields.len()) {
                (_, 0) => FieldsKind::Unit,
                (true, _) => FieldsKind::Named,
                (false, _) => FieldsKind::Unnamed,
            }))
        ),
        Data::Union{..} => panic!("unsupported"),
    }
}

fn generator_for_enum<F, W, V>(varname: &'static str, sep: Sep, name: &Ident, data_enum: &DataEnum, f: F, w: W, v: V) -> Tokens
where
    F: for <'a> Fn(&'a FieldExt) -> Tokens + Copy,
    W: for <'a> Fn(Tokens) -> Tokens,
    V: for <'a> Fn(Tokens, Option<&'a Variant>, FieldsKind) -> Tokens,
{
    let variants = &data_enum.variants;

    let arms = variants.iter().map(|variant| {
        per_struct_field(varname, true, sep, &variant.fields, f, |expr, fields, named| {
            let ident = &variant.ident;

            let one_ref = fields.iter().map(|varient| {
                let ident = &varient.get_match_ident();
                quote! { #ident }
            });

            let (fields_kind, fields_match) = match (variant.fields.len(), named) {
                (0, _) => (FieldsKind::Unit, quote!()),
                (_, false) => (FieldsKind::Unnamed, quote!(( #(#one_ref),* ))),
                (_, true) => (FieldsKind::Named, (quote!({ #(#one_ref),* }))),
            };

            let expr = v(expr, Some(variant), fields_kind);
            quote! {
                #name::#ident #fields_match => {
                    #expr
                }
            }
        })
    });

    let varname = Ident::new(varname, Span::call_site());

    w(quote!(
        match #varname {
            #(#arms),*
        }
    ))
}

#[derive(Clone)]
struct FieldExt<'a> {
    field: &'a syn::Field,
    idx: usize,
    named: bool,
    from_enum: bool,
    var_name: &'static str,
}

impl<'a> FieldExt<'a> {
    fn new(var_name: &'static str, from_enum: bool, field: &'a syn::Field, idx: usize, named: bool) -> FieldExt<'a> {
        FieldExt { field, idx, named, from_enum, var_name }
    }

    #[allow(unused)]
    fn name(&self) -> String {
        if self.named {
            self.field.ident.as_ref().unwrap().to_string()
        } else {
            format!("{}", self.idx)
        }
    }

    #[allow(unused)]
    fn access_expr(&self) -> Tokens {
        if self.from_enum {
            let ident = self.get_match_ident();
            quote! { #ident }
        } else {
            self.field_access_expr()
        }
    }

    fn access_expr_ref(&self) -> Tokens {
        if self.from_enum {
            let ident = self.get_match_ident();
            quote! { #ident }
        } else {
            let expr = self.field_access_expr();
            quote! { &#expr }
        }
    }

    #[allow(unused)]
    fn type_(&self) -> &syn::Type {
        &self.field.ty
    }

    #[allow(unused)]
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.field.attrs
    }

    fn field_access_expr(&self) -> Tokens {
        let self_var_name = Ident::new(self.var_name, Span::call_site());
        if self.named {
            let ident = &self.field.ident;
            quote! { #self_var_name.#ident }
        } else {
            let idx = syn::Index::from(self.idx);
            quote! { #self_var_name.#idx }
        }
    }

    fn get_match_ident(&self) -> Ident {
        if self.named {
            self.field.ident.clone().unwrap()
        } else {
            Ident::new(&format!("f{}", self.idx), Span::call_site())
        }
    }
}
