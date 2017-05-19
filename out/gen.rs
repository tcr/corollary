mod Language_C_Parser_Lexer {
    struct AlexReturn(AlexEOF, AlexError, AlexInput, AlexSkip, AlexInput, isize, AlexToken, AlexInput, isize, a);

    struct AlexLastAcc(AlexNone, AlexLastAcc, isize, AlexInput, isize, AlexLastSkip, AlexInput, isize);

    struct AlexAcc(AlexAccNone, AlexAcc, isize, AlexAccSkip, AlexAccPred, isize, AlexAccPred(user), AlexAcc(user), AlexAccSkipPred, AlexAccPred(user), AlexAcc(user));


}

mod Language_C_Parser_Parser {
    struct HappyAbsSyn(HappyTerminal, CToken, HappyErrorToken, isize, HappyAbsSyn7, CTranslUnit, HappyAbsSyn8, Reversed(Vec<CExtDecl>), HappyAbsSyn9, CExtDecl, HappyAbsSyn10, CFunDef, HappyAbsSyn11, CDeclr, HappyAbsSyn12, CStat, HappyAbsSyn15, (), HappyAbsSyn17, Reversed(Vec<CBlockItem>), HappyAbsSyn18, CBlockItem, HappyAbsSyn21, Reversed(Vec<Ident>), HappyAbsSyn26, CAsmStmt, HappyAbsSyn27, Maybe(CTypeQual), HappyAbsSyn28, Vec<CAsmOperand>, HappyAbsSyn29, Reversed(Vec<CAsmOperand>), HappyAbsSyn30, CAsmOperand, HappyAbsSyn31, Reversed(Vec<CStrLit>), HappyAbsSyn32, CDecl, HappyAbsSyn33, Reversed(Vec<CDecl>), HappyAbsSyn35, (Maybe(CStrLit), Vec<CAttr>), HappyAbsSyn37, Vec<CDeclSpec>, HappyAbsSyn38, Reversed(Vec<CDeclSpec>), HappyAbsSyn39, CDeclSpec, HappyAbsSyn40, CStorageSpec, HappyAbsSyn42, CTypeSpec, HappyAbsSyn50, CStructUnion, HappyAbsSyn51, Located(CStructTag), HappyAbsSyn56, (Maybe(CDeclr), Maybe(CExpr)), HappyAbsSyn58, CEnum, HappyAbsSyn59, Reversed(Vec<(Ident, Maybe(CExpr))>), HappyAbsSyn60, (Ident, Maybe(CExpr)), HappyAbsSyn61, CTypeQual, HappyAbsSyn62, Reversed(Vec<CTypeQual>), HappyAbsSyn63, CDeclrR, HappyAbsSyn64, Maybe(CStrLit), HappyAbsSyn79, (Vec<CDecl>, Bool), HappyAbsSyn85, fn(CDeclrR) -> CDeclrR, HappyAbsSyn90, CInit, HappyAbsSyn91, Maybe(CInit), HappyAbsSyn92, Reversed(CInitList), HappyAbsSyn93, Vec<CDesignator>, HappyAbsSyn94, Reversed(Vec<CDesignator>), HappyAbsSyn95, CDesignator, HappyAbsSyn97, CExpr, HappyAbsSyn100, Reversed(Vec<CExpr>), HappyAbsSyn102, Located(CUnaryOp), HappyAbsSyn116, Located(CAssignOp), HappyAbsSyn119, Maybe(CExpr), HappyAbsSyn122, CConst, HappyAbsSyn123, CStrLit, HappyAbsSyn124, Reversed(Vec<CString>), HappyAbsSyn125, Ident, HappyAbsSyn126, Vec<CAttr>, HappyAbsSyn129, Reversed(Vec<CAttr>), HappyAbsSyn130, Maybe(CAttr));

    struct Located(L, a, Position);

    struct CDeclrR(CDeclrR, Maybe(Ident), Reversed(Vec<CDerivedDeclr>), Maybe(CStrLit), Vec<CAttr>, NodeInfo);

    struct HappyStk(HappyStk, a, HappyStk(a));


}



fn main() { /* demo */ }
