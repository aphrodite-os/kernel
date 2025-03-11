use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, Signature, Token};

struct KernelItemNameInput {
    item: syn::Ident,
}

impl Parse for KernelItemNameInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item: syn::Ident = input.parse()?;
        Ok(KernelItemNameInput { item })
    }
}

fn to_tokens(signature: Signature, tokens: &mut proc_macro2::TokenStream) {
    let ts = tokens;
    signature.constness.to_tokens(ts);
    signature.asyncness.to_tokens(ts);
    signature.unsafety.to_tokens(ts);
    signature.abi.to_tokens(ts);
    signature.fn_token.to_tokens(ts);
    signature.generics.to_tokens(ts);
    signature.paren_token.surround(ts, |tokens| {
        signature.inputs.to_tokens(tokens);
        if let Some(variadic) = &signature.variadic {
            if !signature.inputs.empty_or_trailing() {
                <Token![,]>::default().to_tokens(tokens);
            }
            variadic.to_tokens(tokens);
        }
    });
    signature.output.to_tokens(ts);
    signature.generics.where_clause.to_tokens(ts);
}

fn to_token_stream(signature: Signature) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    to_tokens(signature, &mut tokens);
    tokens
}

/// Implement a kernel item.
#[proc_macro_attribute]
pub fn kernel_item(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name: KernelItemNameInput = syn::parse_macro_input!(attr);
    let item_name = name.item;

    let input_fn = syn::parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.clone().sig.ident;
    let fn_sig = to_token_stream(input_fn.clone().sig);

    quote! {
        /// The #item_name kernel item.
        #[allow(non_upper_case_globals)]
        pub const #item_name: #fn_sig = #fn_name;

        #input_fn
    }
    .into()
}
