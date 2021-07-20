#![crate_type = "proc-macro"]
#![recursion_limit = "250"]

extern crate proc_macro;

use std::{process::Command};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use syn::{DeriveInput};
use quote::quote;

mod derive;

#[proc_macro_derive(Casserole, attributes(casserole))]
pub fn derive_casserole(input: TokenStream) -> TokenStream {
    derive_wrap(input, |input, ()| derive::derive_object_casserole("casserole", &input), ())
}

#[proc_macro]
pub fn derive_casserole_prelude(input: TokenStream) -> TokenStream {
    derive_wrap(input, |input, ()| derive::derive_object_casserole("crate", &input), ())
}

fn derive_wrap<F, P, T>(input: TokenStream, f: F, p: P) -> TokenStream
    where F: for<'a> FnOnce(&'a DeriveInput, P) -> T, T: quote::ToTokens
{
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let b = f(&input, p);
    let res = quote!(#b);

    if let Some((_, value)) =
        std::env::vars().find(|(key, _)| key.as_str() == "DERIVE_SAVE_DIR")
    {
        let dir = std::path::Path::new(value.as_str());
        tokens_to_rustfmt_file(&dir.join(format!("derive_{}.rs", name)), &res);
    }

    res.into()
}

fn tokens_to_rustfmt_file(filename: &std::path::Path, expanded: &Tokens) {
    let mut file = std::fs::File::create(&filename).unwrap();
    use std::io::Write;
    file.write_all(format!("{}", expanded).as_bytes()).unwrap();
    Command::new("rustfmt")
        .args(&[filename])
        .output()
        .expect("failed to execute process");
}
