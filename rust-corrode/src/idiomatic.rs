#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::Rust::AST;

pub fn itemIdioms<a>(_0: Rust::Item) -> Rust::Item {
    match (_0) {
        Rust::Item(attrs, vis, Rust::Function(fattrs, name, formals, ret, b)) => {
            Rust::Item(attrs, vis, (Rust::Function(fattrs, name, formals, ret, (tailBlock(b)))))
        },
        i => {
            i
        },
    }
}

pub fn tailBlock<a>(_0: Rust::Block) -> Rust::Block {
    match (_0) {
        Rust::Block(b, Some(/* TODO ViewPattern */ tailExpr)) => {
            Rust::Block(b, e)
        },
        Rust::Block(/* TODO ViewPattern */ unsnoc, None) => {
            Rust::Block(b, e)
        },
        b => {
            b
        },
    }
}

pub fn tailExpr<a>(_0: Rust::Expr) -> Option<Option<Rust::Expr>> {
    match (_0) {
        Rust::Return(e) => {
            Some(e)
        },
        Rust::BlockExpr(b) => {
            Some((Some((Rust::BlockExpr((tailBlock(b)))))))
        },
        Rust::IfThenElse(c, t, f) => {
            Some((Some((Rust::IfThenElse(c, (tailBlock(t)), (tailBlock(f)))))))
        },
        _ => {
            None
        },
    }
}

pub fn unsnoc<a>(_0: Vec<a>) -> Option<(Vec<a>, a)> {
    match (_0) {
        [] => {
            None
        },
        [x, xs] => {
            match unsnoc(xs) {
                Some((a, b)) => {
                    Some((__op_concat(x, a), b))
                },
                None => {
                    Some((vec![], x))
                },
            }
        },
    }
}



