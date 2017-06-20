#![allow(unused_imports)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use syn::{ Ident, Body, Variant, VariantData };
use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};

#[proc_macro_derive(CNodeable)]
pub fn cnodeable(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_cnodeable(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_cnodeable(ast: &syn::MacroInput) -> quote::Tokens {
    match ast.body {
        Body::Enum(ref variants) => {
            let arms = variants.iter().map(|var| {
                // get args, opaque everything except for node, return node
                let out: Vec<_> = if let &syn::VariantData::Tuple(ref inner) = &var.data {
                    inner.iter().map(|item| {
                        let mut tokens = Tokens::new();
                        item.to_tokens(&mut tokens);
                        let arg = tokens.to_string();
                        if arg == "a" {
                            "node"
                        } else {
                            "_"
                        }
                    }).collect()
                } else {
                    unreachable!("Expected enum tuple.");
                };
                let name = &var.ident;
                let mut args = Tokens::new();
                syn::parse_type(&format!("({})", out.join(", "))).unwrap().to_tokens(&mut args);
                if out.iter().position(|&x| x == "node").is_none() {
                    if name == "CBuiltinExpr" {
                        quote! {
                            #name ( node ) => nodeInfo(*node),
                        }
                    } else {
                        quote! {
                            #name ( node ) => nodeInfo(node),
                        }
                    }
                } else {
                    quote! {
                        #name #args => node,
                    }
                }
            })
            .collect::<Vec<_>>();

            let name = &ast.ident;
            quote! {
                impl CNode for #name<NodeInfo> {
                    fn nodeInfo(self) -> NodeInfo {
                        match self {
                            #(#arms)*
                        }
                    }
                }

                impl Pos for #name<NodeInfo> {
                    fn posOf(self) -> Position {
                        posOf(nodeInfo(self))
                    }
                }
            }
        }
        Body::Struct(ref var) => {
            // get args, opaque everything except for node, return node
            let node_pos: Option<usize> = if let &syn::VariantData::Tuple(ref inner) = var {
                inner.iter().position(|item| {
                    let mut tokens = Tokens::new();
                    item.to_tokens(&mut tokens);
                    let arg = tokens.to_string();
                    arg == "a" || arg == "pub a"
                })
            } else {
                unreachable!("Expected struct tuple.");
            };

            let mut args = Tokens::new();
            syn::parse_expr(&(
                if node_pos.is_none() { //&& var.len() == 1 {
                    format!("nodeInfo(self.0)")
                } else if let Some(pos) = node_pos {
                    format!("self.{}", pos)
                } else {
                    unreachable!("Expected struct entry to be valid");
                }
            )).unwrap().to_tokens(&mut args);

            let name = &ast.ident;
            quote! {
                impl CNode for #name<NodeInfo> {
                    fn nodeInfo(self) -> NodeInfo {
                        #args
                    }
                }

                impl Pos for #name<NodeInfo> {
                    fn posOf(self) -> Position {
                        posOf(nodeInfo(self))
                    }
                }
            }
        }
    }
}