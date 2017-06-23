#![feature(proc_macro)]
#![allow(unused_imports)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use syn::{ Ident, Body, Variant, VariantData };
use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};

#[proc_macro]
pub fn refute(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let mut ast = syn::parse_item(&input).unwrap();

    match &mut ast.node {
        &mut syn::ItemKind::Fn(ref mut decl, _, _, _, _, ref mut body) => {
            let mut pat_expand = vec![];
            for (i, arg) in decl.inputs.iter_mut().enumerate() {
                match arg {
                    &mut syn::FnArg::Captured(ref mut pat, _) => {
                        pat_expand.push(pat.clone());
                        *pat = syn::Pat::Ident(syn::BindingMode::ByValue(syn::Mutability::Immutable),
                            syn::Ident::new(format!("_{}", i)), None);
                    }
                    _ => {
                        println!("TODO");
                    }
                }
            }

            let body_inner: syn::Block = (**body).clone();



            *body = Box::new(syn::Block {
                stmts: vec![
                    syn::Stmt::Expr(Box::new(syn::Expr {
                        node: syn::ExprKind::Match(
                            Box::new(syn::parse_expr(&format!("({})", 
                                (0..pat_expand.len()).map(|x| format!("_{}", x)).collect::<Vec<_>>().join(", ")
                            )).unwrap()),
                            vec![
                                syn::Arm {
                                    attrs: vec![],
                                    pats: if pat_expand.len() == 1 {
                                        pat_expand
                                    } else {
                                        vec![syn::Pat::Tuple(pat_expand, None)]
                                    },
                                    guard: None,
                                    body: Box::new(syn::Expr {
                                        node: syn::ExprKind::Block(syn::BlockCheckMode::Default, body_inner),
                                        attrs: vec![],
                                    })
                                },
                                syn::Arm {
                                    attrs: vec![],
                                    pats: vec![syn::Pat::Wild],
                                    guard: None,
                                    body: Box::new(syn::parse_expr(r#"panic!("Irrefutable pattern!")"#).unwrap()),
                                }
                            ],
                        ),
                        attrs: vec![],
                    })),
                ],
            });
        }
        _ => {
            panic!("Unexpected item, expected fn");
        }
    }

    let mut args = Tokens::new();
    ast.to_tokens(&mut args);
    
    args.parse().unwrap()
}

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
