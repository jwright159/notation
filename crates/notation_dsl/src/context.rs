use std::sync::RwLock;

use fehler::{throw, throws};
use notation_proto::prelude::{Duration, GUITAR_STRING_NUM};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Ident, LitInt, Token};

lazy_static! {
    static ref CONTEXT: RwLock<Context> = RwLock::new(Context::default());
}

#[derive(Copy, Clone, Debug)]
pub struct FrettedContext {
    pub string_num: usize,
}
impl Default for FrettedContext {
    fn default() -> Self {
        Self {
            string_num: GUITAR_STRING_NUM,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub duration: Duration,
    pub fretted: FrettedContext,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            duration: Duration::default(),
            fretted: FrettedContext::default(),
        }
    }
}

impl Context {
    pub fn _duration() -> Duration {
        CONTEXT.read().unwrap().duration
    }
    pub fn fretted() -> FrettedContext {
        CONTEXT.read().unwrap().fretted
    }
}

impl Context {
    pub fn duration_quote() -> TokenStream {
        let duration = CONTEXT.read().unwrap().duration;
        let ident = format!("{}", duration);
        quote! {
            Duration::from_ident(#ident)
        }
    }
}

pub enum ContextDsl {
    Duration(Ident),
    StringNum(usize),
}

impl Parse for ContextDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        match input.parse::<Ident>()?.to_string().as_str() {
            "duration" => {
                input.parse::<Token![=]>()?;
                Self::Duration(input.parse()?)
            }
            "string_num" => {
                input.parse::<Token![=]>()?;
                let string_num = input.parse::<LitInt>()?.base10_parse::<usize>()?;
                Self::StringNum(string_num)
            }
            _ => throw!(Error::new(input.span(), "Invalid Field")),
        }
    }
}

impl ToTokens for ContextDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Duration(x) => {
                CONTEXT.write().unwrap().duration = Duration::from_ident(x.to_string().as_str());
                let comment = format!("{}", CONTEXT.read().unwrap().duration);
                quote! {
                    ProtoEntry::from(("dsl::context::duration", #comment))
                }
            }
            Self::StringNum(x) => {
                CONTEXT.write().unwrap().fretted.string_num = *x;
                let comment = format!("{}", CONTEXT.read().unwrap().fretted.string_num);
                quote! {
                    ProtoEntry::from(("dsl::context::string_num", #comment))
                }
            }
        });
    }
}
