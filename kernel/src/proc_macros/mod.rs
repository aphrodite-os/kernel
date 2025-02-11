use proc_macro::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{parse::{Parse, ParseStream}, spanned::Spanned, ItemFn, Signature, Token};

struct KernelItemNameInput {
    item: aphrodite_common::KernelItem
}

impl Parse for KernelItemNameInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item: aphrodite_common::KernelItem = input.parse()?;
        Ok(KernelItemNameInput { item })
    }
}

fn to_tokens(signature: Signature, tokens: &mut TokenStream) {
    signature.constness.to_tokens(&mut (*tokens).clone().into());
    signature.asyncness.to_tokens(&mut (*tokens).clone().into());
    signature.unsafety.to_tokens(&mut (*tokens).clone().into());
    signature.abi.to_tokens(&mut (*tokens).clone().into());
    signature.fn_token.to_tokens(&mut (*tokens).clone().into());
    signature.generics.to_tokens(&mut (*tokens).clone().into());
    signature.paren_token.surround(&mut (*tokens).clone().into(), |tokens| {
        signature.inputs.to_tokens(tokens);
        if let Some(variadic) = &signature.variadic {
            if !signature.inputs.empty_or_trailing() {
                <Token![,]>::default().to_tokens(tokens);
            }
            variadic.to_tokens(tokens);
        }
    });
    signature.output.to_tokens(&mut (*tokens).clone().into());
    signature.generics.where_clause.to_tokens(&mut (*tokens).clone().into());
}

fn to_token_stream(signature: Signature) -> TokenStream {
    let mut tokens = proc_macro::TokenStream::new();
    to_tokens(signature, &mut tokens);
    tokens.into()
}

/// Implement a kernel item. 
#[proc_macro_attribute]
pub fn kernel_item(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_name_input: KernelItemNameInput = syn::parse_macro_input!(attr);
    let item_name = format!("{:?}", item_name_input.item);

    let input_fn: ItemFn = syn::parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.clone().sig.ident;
    let fn_sig = to_token_stream(input_fn.clone().sig);

    quote_spanned!(input_fn.span()=>{
        // The #item_name kernel item.
        #[allow(non_upper_case_globals)]
        pub const #item_name: #fn_sig = #fn_name;

        #input_fn
    }).into()
}