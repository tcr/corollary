mod Language_C_Analysis_AstAnalysis {
    fn addExternIdent(ident: EnvMonad) -> EnvMonad {
        /* do */ {
            let action = runOnce(/* do */ {
                        let itype = deferred;
                        let rewrites = lift(asks(itemRewrites));
                        let path = match Map_lookup((Symbol, identToString(ident)), rewrites) {
                                Just(renamed) => {
                                    return((__op_concat("".to_string(), renamed)))
                                },
                                Nothing => {
                                    /* do */ {
                                        {
                                                let name = || {
                                                    applyRenames(ident)
                                                };
                                        };
                                        {
                                                let ty = || {
                                                    (typeMutable(itype), typeRep(itype))
                                                };
                                        };
                                        lift(tell(mempty, {
                                            outputExterns: Map_singleton(name, (mkItem(name, ty)))
                                            }));
                                        return(vec![name])
                                    }
                                },
                            };
                        return((typeToResult(itype, (Rust_Path((Rust_PathSegments(path)))))))
                    });
            addSymbolIdentAction(ident, action)
        }
    }

}



fn main() { /* demo */ }
