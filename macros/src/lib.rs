use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


#[proc_macro_attribute]
pub fn rpcfunc(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let block = item.block;
    let ident = item.sig.ident;
    let args = item.sig.inputs;

    let types: Vec<_> = args
        .clone()
        .into_iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(val) => Some(val.ty),
        })
        .collect();
    assert!(types.len() > 0);

    let vis = item.vis;
    let outp = item.sig.output;

    if types.len() == 1 {
        let arg = types.get(0).unwrap().clone();
        return quote! {
            #vis fn #ident(data: Vec<u8>) -> Result<Vec<u8>, TiRpcError> {
                assert!(data.len() > 0);

                let src = bincode::deserialize::<(#arg,)>(&data)?;
                pub fn of(#args) #outp{
                    #block
                }
                let re = of.call_tuple(src);
                Ok(bincode::serialize(&re)?)
            }
        }
        .into();
    } else {
        return quote! {
            #vis fn #ident(data: Vec<u8>) -> Result<Vec<u8>, TiRpcError> {
                assert!(data.len() > 0);
                let src = bincode::deserialize::<(#(#types),*)>(&data)?;
                pub fn of(#args) #outp{
                    #block
                }
                let re = of.call_tuple(src);
                Ok(bincode::serialize(&re)?)
            }
        }
        .into();
    }
}
