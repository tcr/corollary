use haskell_support::*;

pub enum ParseResult<a> {
    POk<PState, a>,
    PFailed<Vec<String>, Position>
}
pub use self::ParseResult::*;

struct PState(PState<{ /* type record */ }>);

let (P(m)) = |thenP, k| {
    P(|s| { match m(s) {
            POk(s_q, a) => {
                (unP((k(a))))(s_q)
            },
            PFailed(err, pos) => {
                PFailed(err, pos)
            },
        } })
};

pub fn addTypedef(ident: Ident) -> P<()> {
    (P(|s, @, PState, { /* pat record */ }| { POk }(s, {
        tyidents: Set.insert(ident, tyids)
    }, ())))
}

pub fn enterScope() -> P<()> {
    P(|s, @, PState, { /* pat record */ }| { POk }(s, {
        scopes: __op_concat(tyids, ss)
    }, ()))
}

pub fn execParser((P(parser)): P<a>, input: InputStream, pos: Position, builtins: Vec<Ident>, names: Vec<Name>) -> Either<ParseError, (a, Vec<Name>)> {
    match parser(initialState) {
        PFailed(message, errpos) => {
            Left((ParseError((message, errpos))))
        },
        POk(st, result) => {
            Right((result, namesupply(st)))
        },
    }
}

pub fn failP(pos: Position, msg: Vec<String>) -> P<a> {
    P(|_| { PFailed }(msg, pos))
}

pub fn getCurrentPosition() -> P<Position> {
    P(|s, @, PState, { /* pat record */ }| { POk }(s, pos))
}

pub fn getInput() -> P<InputStream> {
    P(|s, @, PState, { /* pat record */ }| { POk }(s, i))
}

pub fn getLastToken() -> P<CToken> {
    P(|s, @, PState, { /* pat record */ }| { POk }(s, tok))
}

pub fn getNewName() -> P<Name> {
    P(seq(|s, @, PState, { /* pat record */ }| { n }, POk(s, {
        namesupply: ns
    }, n)))
}

pub fn getPos() -> P<Position> {
    P(|s, @, PState, { /* pat record */ }| { POk }(s, pos))
}

pub fn getSavedToken() -> P<CToken> {
    P(|s, @, PState, { /* pat record */ }| { POk }(s, tok))
}

pub fn handleEofToken() -> P<()> {
    P(|s| { POk }(s, {
        savedToken: (prevToken(s))
    }, ()))
}

pub fn isTypeIdent(ident: Ident) -> P<bool> {
    P($!(|s, @, PState, { /* pat record */ }| { POk }(s), Set::member(ident, tyids)))
}

pub fn leaveScope() -> P<()> {
    P(|s, @, PState, { /* pat record */ }| { match ss {
            [] => {
                __error!("leaveScope: already in global scope".to_string())
            },
            [tyids, ...ss_q] => {
                POk(s, {
                    tyidents: tyids,
                    scopes: ss_q
                }, ())
            },
        } })
}

pub fn returnP(a: a) -> P<a> {
    P(|s| { POk }(s, a))
}

pub fn setInput(i: InputStream) -> P<()> {
    P(|s| { POk }(s, {
        curInput: i
    }, ()))
}

pub fn setLastToken(__0: CToken) -> P<()> {
    match (__0) {
        CTokEof => {
            P(|s| { POk }(s, {
                savedToken: (prevToken(s))
            }, ()))
        },
        tok => {
            P(|s| { POk }(s, {
                prevToken: tok,
                savedToken: (prevToken(s))
            }, ()))
        },
    }
}

pub fn setPos(pos: Position) -> P<()> {
    P(|s| { POk }(s, {
        curPos: pos
    }, ()))
}

pub fn shadowTypedef(ident: Ident) -> P<()> {
    (P(|s, @, PState, { /* pat record */ }| { POk }(s, {
        tyidents: Set.member(if(ident), Set.delete(tyids(then, ident), tyids(else, tyids)))
    }, ())))
}

