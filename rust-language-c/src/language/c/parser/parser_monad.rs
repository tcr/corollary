use haskell_support::*;

use language_.c._data::error;
use internal_err;
use language_.c._data::position;
use position;
use language_.c._data::input_stream;
use language_.c._data::name;
use name;
use language_.c._data::ident;
use ident;
use language_.c._parser::tokens;
use c_token;
use data::set;
use set;
use qualified;
use data::set;
use as;
use set;
use from_list;

pub enum ParseResult<a> {
    POk(PState, a),
    PFailed(Vec<String>, Position)
}
pub use self::ParseResult::*;

struct PState(PState<TypeRecord /* todo */>);

pub fn addTypedef(ident: Ident) -> P<()> {
    (P(|s, __OP__, PState { /* TODO pat record */ }| { POk(s {
                tyidents: Set::insert(ident, tyids)
            }, ()) }))
}

pub fn enterScope() -> P<()> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { POk(s {
                scopes: __op_concat(tyids, ss)
            }, ()) })
}

pub fn execParser(P(parser): P<a>, input: InputStream, pos: Position, builtins: Vec<Ident>, names: Vec<Name>) -> Either<ParseError, (a, Vec<Name>)> {
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
    P(|_| { PFailed(msg, pos) })
}

pub fn getCurrentPosition() -> P<Position> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { POk(s, pos) })
}

pub fn getInput() -> P<InputStream> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { POk(s, i) })
}

pub fn getLastToken() -> P<CToken> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { POk(s, tok) })
}

pub fn getNewName() -> P<Name> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { seq(n, POk(s {
                namesupply: ns
            }, n)) })
}

pub fn getPos() -> P<Position> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { POk(s, pos) })
}

pub fn getSavedToken() -> P<CToken> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { POk(s, tok) })
}

pub fn handleEofToken() -> P<()> {
    P(|s| { POk(s {
                savedToken: (prevToken(s))
            }, ()) })
}

pub fn isTypeIdent(ident: Ident) -> P<bool> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { __op_TODO_dollarnot(POk(s), Set::member(ident, tyids)) })
}

pub fn leaveScope() -> P<()> {
    P(|s, __OP__, PState { /* TODO pat record */ }| { match ss {
            [] => {
                __error!("leaveScope: already in global scope".to_string())
            },
            [tyids, ss_q] => {
                POk(s {
                        tyidents: tyids,
                        scopes: ss_q
                    }, ())
            },
        } })
}

pub fn returnP(a: a) -> P<a> {
    P(|s| { POk(s, a) })
}

pub fn setInput(i: InputStream) -> P<()> {
    P(|s| { POk(s {
                curInput: i
            }, ()) })
}

pub fn setLastToken(__0: CToken) -> P<()> {
    match (__0) {
        CTokEof => {
            P(|s| { POk(s {
                        savedToken: (prevToken(s))
                    }, ()) })
        },
        tok => {
            P(|s| { POk(s {
                        prevToken: tok,
                        savedToken: (prevToken(s))
                    }, ()) })
        },
    }
}

pub fn setPos(pos: Position) -> P<()> {
    P(|s| { POk(s {
                curPos: pos
            }, ()) })
}

pub fn shadowTypedef(ident: Ident) -> P<()> {
    (P(|s, __OP__, PState { /* TODO pat record */ }| { POk(s {
                tyidents: Set::member(__TODO_if(ident), Set::delete(tyids(then, ident), tyids(__TODO_else, tyids)))
            }, ()) }))
}

pub fn thenP(P(m): P<a>, k: fn(a) -> P<b>) -> P<b> {
    P(|s| { match m(s) {
            POk(s_q, a) => {
                (unP((k(a))))(s_q)
            },
            PFailed(err, pos) => {
                PFailed(err, pos)
            },
        } })
}

