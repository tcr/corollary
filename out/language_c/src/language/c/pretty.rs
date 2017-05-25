use haskell_support::*;

pub fn attrlistP(__0: Vec<CAttr>) -> Doc {
    match (__0) {
        [] => {
            empty
        },
        attrs => {
            <>(text("__attribute__".to_string()), parens((parens((hcat(punctuate(comma, map(pretty)(attrs))))))))
        },
    }
}

pub fn binPrec(__0: CBinaryOp) -> isize {
    match (__0) {
        CMulOp => {
            20
        },
        CDivOp => {
            20
        },
        CRmdOp => {
            20
        },
        CAddOp => {
            19
        },
        CSubOp => {
            19
        },
        CShlOp => {
            18
        },
        CShrOp => {
            18
        },
        CLeOp => {
            17
        },
        CGrOp => {
            17
        },
        CLeqOp => {
            17
        },
        CGeqOp => {
            17
        },
        CEqOp => {
            16
        },
        CNeqOp => {
            16
        },
        CAndOp => {
            15
        },
        CXorOp => {
            14
        },
        COrOp => {
            13
        },
        CLndOp => {
            12
        },
        CLorOp => {
            11
        },
    }
}

pub fn identP() -> Doc {
    text(identToString)
}

pub fn ifP(flag: bool, doc: Doc) -> Doc {
    if(flag, then, doc, else, empty)
}

pub fn ii() -> Doc {
    nest(4)
}

pub fn maybeP() -> Doc {
    maybe(empty)
}

pub fn mlistP(pp: fn(Vec<p>) -> Doc, xs: Vec<p>) -> Doc {
    maybeP(pp, (if(null, xs, then, None, else, Some, xs)))
}

pub fn parenPrec(prec: isize, prec2: isize, t: Doc) -> Doc {
    <=(if(prec), prec2(then, t, else, parens, t))
}

pub fn prettyDeclr(show_attrs: bool, prec: isize, (CDeclr(name, derived_declrs, asmname, cattrs, _)): CDeclr) -> Doc {
    <+>(ppDeclr(prec, (reverse(derived_declrs))), <+>(prettyAsmName(asmname), ifP(show_attrs, (attrlistP(cattrs)))))
}

pub fn prettyUsingInclude((CTranslUnit(edecls, _)): CTranslUnit) -> Doc {
    $$(includeWarning(headerFiles), (vcat(map((either(includeHeader, pretty)), mappedDecls))))
}

