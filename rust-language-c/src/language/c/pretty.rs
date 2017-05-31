use haskell_support::*;

use Data::List;
use partition;
use Data::Set;
use Text::PrettyPrint::HughesPJ;
use Debug::Trace;
use Language::C::Data;
use Language::C::Syntax;

pub fn attrlistP(__0: Vec<CAttr>) -> Doc {
    match (__0) {
        [] => {
            empty
        },
        attrs => {
            __op_ne(text("__attribute__".to_string()), parens((parens((hcat(punctuate(comma, map(pretty)(attrs))))))))
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
    __TODO_if(flag, then, doc, __TODO_else, empty)
}

pub fn ii() -> Doc {
    nest(4)
}

pub fn maybeP() -> Doc {
    maybe(empty)
}

pub fn mlistP(pp: fn(Vec<p>) -> Doc, xs: Vec<p>) -> Doc {
    maybeP(pp, (__TODO_if(null, xs, then, None, __TODO_else, Some, xs)))
}

pub fn parenPrec(prec: isize, prec2: isize, t: Doc) -> Doc {
    (__TODO_if(prec) <= prec2(then, t, __TODO_else, parens, t))
}

pub fn prettyDeclr(show_attrs: bool, prec: isize, CDeclr(name, derived_declrs, asmname, cattrs, _): CDeclr) -> Doc {
    __op_doc_conat(ppDeclr(prec, (reverse(derived_declrs))), __op_doc_conat(prettyAsmName(asmname), ifP(show_attrs, (attrlistP(cattrs)))))
}

pub fn prettyUsingInclude(CTranslUnit(edecls, _): CTranslUnit) -> Doc {
    __op_line_something(includeWarning(headerFiles), (vcat(map((either(includeHeader, pretty)), mappedDecls))))
}

