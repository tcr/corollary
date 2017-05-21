mod Language_C_Analysis_AstAnalysis {
    enum StmtCtx {
        FunCtx(VarDecl),
        LoopCtx,
        SwitchCtx
    }

    #[derive(Debug, Eq)]
    enum ExprSide {
        LValue,
        RValue
    }

    fn analyseAST((CTranslUnit(decls, _file_node)): m) -> m {
        /* do */ {

            mapRecoverM_(analyseExt, decls);
            __op_bind(getDefTable, Lambda((not((inFileScope(dt)))))(error("Internal Error: Not in filescope after analysis".to_string())));
            liftM(globalDefs, getDefTable);
        }
    }

    fn analyseExt(__0: m) -> m {
        match (__0) {
            CAsmExt(asm, _) => { handleAsmBlock(asm) },
            CFDefExt(fundef) => { analyseFunDef(fundef) },
            CDeclExt(decl) => { analyseDecl(False, decl) },
        }
    }

}



fn main() { /* demo */ }
