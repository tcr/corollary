use haskell_support::*;

pub enum ParseResult<a> {
    POk(PState, a),
    PFailed(Vec<String>, Position)
}
pub use self::ParseResult::*;

struct PState(PState<TypeRecord /* todo */>);

pub fn addTypedef(ident: Ident) -> P<()> {
    (P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s, {
        tyidents: Set.insert(ident, tyids)
    }, ())))
}

pub fn enterScope() -> P<()> {
    P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s, {
        scopes: __op_concat(tyids, ss)
    }, ()))
}

pub fn execParser(P(parser): P<a>, input: InputStream, pos: Position, builtins: Vec<Ident>, names: Vec<Name>) -> Either<ParseError, (a, Vec<Name>)> {
    match parser(initialState) {
        PFailed | message | errpos => {
            Left((ParseError((message, errpos))))
        },
        POk | st | result => {
            Right((result, namesupply(st)))
        },
    }
}

pub fn failP(pos: Position, msg: Vec<String>) -> P<a> {
    P(|_| { <Expr::Dummy> }(PFailed, msg, pos))
}

pub fn getCurrentPosition() -> P<Position> {
    P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s, pos))
}

pub fn getInput() -> P<InputStream> {
    P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s, i))
}

pub fn getLastToken() -> P<CToken> {
    P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s, tok))
}

pub fn getNewName() -> P<Name> {
    P(seq(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(n), POk(s, {
        namesupply: ns
    }, n)))
}

pub fn getPos() -> P<Position> {
    P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s, pos))
}

pub fn getSavedToken() -> P<CToken> {
    P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s, tok))
}

pub fn handleEofToken() -> P<()> {
    P(|s| { <Expr::Dummy> }(POk, s, {
        savedToken: (prevToken(s))
    }, ()))
}

pub fn isTypeIdent(ident: Ident) -> P<bool> {
    P(__op_TODO_dollarnot(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s), Set::member(ident, tyids)))
}

pub fn leaveScope() -> P<()> {
    P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(match ss {
            [] => {
                __error!("leaveScope: already in global scope".to_string())
            },
            [tyids, ss_q] => {
                POk(s, {
                    tyidents: tyids,
                    scopes: ss_q
                }, ())
            },
        }))
}

pub fn returnP(a: a) -> P<a> {
    P(|s| { <Expr::Dummy> }(POk, s, a))
}

pub fn setInput(i: InputStream) -> P<()> {
    P(|s| { <Expr::Dummy> }(POk, s, {
        curInput: i
    }, ()))
}

pub fn setLastToken(__0: CToken) -> P<()> {
    match (__0) {
        CTokEof => {
            P(|s| { <Expr::Dummy> }(POk, s, {
                savedToken: (prevToken(s))
            }, ()))
        },
        tok => {
            P(|s| { <Expr::Dummy> }(POk, s, {
                prevToken: tok,
                savedToken: (prevToken(s))
            }, ()))
        },
    }
}

pub fn setPos(pos: Position) -> P<()> {
    P(|s| { <Expr::Dummy> }(POk, s, {
        curPos: pos
    }, ()))
}

pub fn shadowTypedef(ident: Ident) -> P<()> {
    (P(|s, __OP__, PState, { /* pat record */ }| { <Expr::Dummy> }(POk, s, {
        tyidents: Set.member(__TODO_if(ident), Set.delete(tyids(then, ident), tyids(__TODO_else, tyids)))
    }, ())))
}

pub fn thenP(P(m): P<a>, k: fn(a) -> P<b>) -> P<b> {
    P(|s| { <Expr::Dummy> }(match m(s) {
            POk | s_q | a => {
                (unP((k(a))))(s_q)
            },
            PFailed | err | pos => {
                PFailed(err, pos)
            },
        }))
}

