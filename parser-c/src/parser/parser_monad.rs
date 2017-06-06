// Original file: "ParserMonad.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::Error;
// use internalErr;
// use Language::C::Data::Position;
// use Position;
// use Language::C::Data::InputStream;
// use Language::C::Data::Name;
// use Name;
// use Language::C::Data::Ident;
// use Ident;
// use Language::C::Parser::Tokens;
// use CToken;
// use Control::Applicative;
// use Applicative;
// use Control::Monad;
// use liftM;
// use Data::Set;
// use Set;
// use Data::Set;

use data::position::Position;
use parser::tokens::*;
use data::input_stream::*;
use data::ident::Ident;
use data::name::Name;
use data::error::*;

pub struct ParseError(pub (Vec<String>, Position));

// instance Show ParseError where
//     show (ParseError (msgs,pos)) = showErrorInfo "Syntax Error !" (ErrorInfo LevelError pos msgs)

pub enum ParseResult<a> {
    POk(PState, a),
    PFailed(Vec<String>, Position),
}
pub use self::ParseResult::*;

pub struct PState {
    curPos: Position,
    curInput: InputStream,
    prevToken: CToken,
    savedToken: CToken,
    namesupply: Vec<Name>,
    tyidents: Set<Ident>,
    scopes: Vec<Set<Ident>>,
}
fn curPos(a: PState) -> Position {
    a.curPos
}
fn curInput(a: PState) -> InputStream {
    a.curInput
}
fn prevToken(a: PState) -> CToken {
    a.prevToken
}
fn savedToken(a: PState) -> CToken {
    a.savedToken
}
fn namesupply(a: PState) -> Vec<Name> {
    a.namesupply
}
fn tyidents(a: PState) -> Set<Ident> {
    a.tyidents
}
fn scopes(a: PState) -> Vec<Set<Ident>> {
    a.scopes
}

pub struct P<a> {
    unP: Box<fn(PState) -> ParseResult<a>>,
}
fn unP(a: P) -> Box<fn(PState) -> ParseResult<a>> {
    a.unP
}

pub fn execParser<a>(P(parser): P<a>,
                     input: InputStream,
                     pos: Position,
                     builtins: Vec<Ident>,
                     names: Vec<Name>)
                     -> Either<ParseError, (a, Vec<Name>)> {

    let initialState = PState {
        curPos: pos,
        curInput: input,
        prevToken: internalErr("CLexer.execParser: Touched undefined token!".to_string()),
        savedToken: internalErr("CLexer.execParser: Touched undefined token (safed token)!"
                                    .to_string()),
        namesupply: names,
        tyidents: Set::fromList(builtins),
        scopes: vec![],
    };

    match parser(initialState) {
        PFailed(message, errpos) => Left((ParseError((message, errpos)))),
        POk(st, result) => Right((result, namesupply(st))),
    }
}

pub fn returnP<a>(a: a) -> P<a> {
    P(box |s| POk(s, a))
}

pub fn thenP<a, b>(P(m): P<a>, k: fn(a) -> P<b>) -> P<b> {
    P(box |s| match m(s) {
          POk(s_q, a) => (unP((k(a))))(s_q),
          PFailed(err, pos) => PFailed(err, pos),
      })
}

pub fn failP<a>(pos: Position, msg: Vec<String>) -> P<a> {
    P(box |_| PFailed(msg, pos))
}

pub fn getNewName() -> P<Name> {
    P(box |s| seq(n, POk(__assign!(s, { namesupply: ns }), n)))
}

pub fn setPos(pos: Position) -> P<()> {
    P(box |s| POk(__assign!(s, { curPos: pos }), ()))
}

pub fn getPos() -> P<Position> {
    P(box |s| POk(s, s.curPos.clone()))
}

pub fn addTypedef(ident: Ident) -> P<()> {
    P(box |s| POk(s { tyidents: Set::insert(ident, s.tyidents.clone()) }, ()))
}

pub fn shadowTypedef(ident: Ident) -> P<()> {
    P(box |s| {
        POk(__assign!(s, {
                tyidents: (if Set::member(ident, tyids) {
                    Set::delete(ident, s.tyidents.clone())
                } else {
                    tyids
                }),
            }),
            ())
    })
}

pub fn isTypeIdent(ident: Ident) -> P<bool> {
    P(box |s| POk(__op_TODO_dollarnot(s, Set::member(ident, tyids))))
}

pub fn enterScope() -> P<()> {
    P(box |s| POk(__assign!(s, { scopes: __op_concat(s.tyidents.clone(), s.scopes.clone()) }), ()))
}

pub fn leaveScope() -> P<()> {
    P(box |s| match s.scopes.clone() {
          [] => __error!("leaveScope: already in global scope".to_string()),
          [tyids, ss_q] => {
              POk(__assign!(s, {
                      tyidents: tyids,
                      scopes: ss_q,
                  }),
                  ())
          }
      })
}

pub fn getInput() -> P<InputStream> {
    P(|s, __OP__, PState {}| POk(s, i))
}

pub fn setInput(i: InputStream) -> P<()> {
    P(box |s| POk(__assign!(s, { curInput: i }), ()))
}

pub fn getLastToken() -> P<CToken> {
    P(box |s| POk(s, s.prevToken.clone()))
}

pub fn getSavedToken() -> P<CToken> {
    P(box |s| POk(s, s.savedToken.clone()))
}

pub fn setLastToken(_0: CToken) -> P<()> {
    match (_0) {
        CTokEof => P(box |s| POk(__assign!(s, { savedToken: (prevToken(s)) }), ())),
        tok => {
            P(box |s| {
                  POk(__assign!(s, {
                          prevToken: tok,
                          savedToken: (prevToken(s)),
                      }),
                      ())
              })
        }
    }
}

pub fn handleEofToken() -> P<()> {
    P(|s| POk(s { savedToken: (prevToken(s)) }, ()))
}

pub fn getCurrentPosition() -> P<Position> {
    P(|s, __OP__, PState {}| POk(s, pos))
}
