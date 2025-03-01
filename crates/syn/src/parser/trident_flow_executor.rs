use syn::parse::Error as ParseError;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::ItemImpl;

use crate::types::trident_flow_executor::TridentFlowExecutorImpl;

pub fn parse_trident_flow_executor(
    _attr: proc_macro2::TokenStream,
    input: &ItemImpl,
) -> ParseResult<TridentFlowExecutorImpl> {
    let type_name = input.self_ty.clone();
    let generics = input.generics.clone();

    let mut init_method = None;
    let mut flow_methods = Vec::new();

    // Collect init and flow methods
    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            // First check for init methods
            if method.attrs.iter().any(|attr| attr.path().is_ident("init")) {
                if init_method.is_some() {
                    return Err(ParseError::new(
                        method.span(),
                        "Multiple #[init] methods found. Only one is allowed.",
                    ));
                }
                init_method = Some(method.sig.ident.clone());
                continue;
            }

            // Then check for flow methods
            if method.attrs.iter().any(|attr| attr.path().is_ident("flow")) {
                // Only check for ignore if it's a flow method
                let is_ignored = method
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("flow_ignore"));
                if !is_ignored {
                    flow_methods.push(method.sig.ident.clone());
                }
            }
        }
    }

    Ok(TridentFlowExecutorImpl {
        type_name,
        impl_block: input.items.clone(),
        flow_methods,
        init_method,
        generics,
    })
}
