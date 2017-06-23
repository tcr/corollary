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
use std::boxed::FnBox;
use std::rc::Rc;

pub struct ParseError(pub (Vec<String>, Position));

// instance Show ParseError where
//     show (ParseError (msgs,pos)) = showErrorInfo "Syntax Error !" (ErrorInfo LevelError pos msgs)

#[derive(Clone)]
pub enum ParseResult<a> {
    POk(PState, a),
    PFailed(Vec<String>, Position),
}
pub use self::ParseResult::*;

#[derive(Clone)]
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

pub struct P<a>(pub Rc<Box<Fn(PState) -> ParseResult<a>>>);
pub fn unP<a>(p: P<a>) -> Rc<Box<Fn(PState) -> ParseResult<a>>> {
    p.0
}

impl<a> P<a> {
    fn with(item: Box<Fn(PState) -> ParseResult<a>>) -> P<a> {
        P(Rc::new(item))
    }
}

impl<a> Clone for P<a> {
    fn clone(&self) -> Self {
        P(self.0.clone())
    }
}

impl<A: 'static> From<A> for P<A> {
    fn from(item: A) -> P<A> {
        P::with(box |state| POk(state, item))
    }
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

pub fn returnP<a: 'static>(a: a) -> P<a> {
    P::with(box move |s| POk(s, a))
}

pub fn thenP<a: 'static, b: 'static>(P(m): P<a>, k: Box<Fn(a) -> P<b>>) -> P<b> {
    P::with(box move |s| match m(s) {
          POk(s_q, a) => (unP((k(a))))(s_q),
          PFailed(err, pos) => PFailed(err, pos),
      })
}

pub fn failP<a>(pos: Position, msg: Vec<String>) -> P<a> {
    P::with(box |_| PFailed(msg, pos))
}

pub fn getNewName() -> P<Name> {
    P::with(box |s: PState| {
        let mut ns = s.namesupply.clone();
        let n = ns.remove(0);
        seq(n.clone(), POk(__assign!(s, { namesupply: ns }), n))
    })
}

pub fn setPos(pos: Position) -> P<()> {
    P::with(box |s: PState| POk(__assign!(s, { curPos: pos }), ()))
}

pub fn getPos() -> P<Position> {
    P::with(box |s: PState| POk(s.clone(), s.curPos.clone()))
}

pub fn addTypedef(ident: Ident) -> P<()> {
    P::with(box |s: PState| POk(__assign!(s, { tyidents: Set::insert(ident, s.tyidents.clone()) }), ()))
}

pub fn shadowTypedef(ident: Ident) -> P<()> {
    P::with(box |s: PState| {
        POk(__assign!(s, {
                tyidents: (if Set::member(ident, s.tyidents.clone()) {
                    Set::delete(ident, s.tyidents.clone())
                } else {
                    s.tyidents.clone()
                }),
            }),
            ())
    })
}

pub fn isTypeIdent(mut ident: Ident) -> P<bool> {
    P::with(box |s: PState| POk(s.clone(), Set::member(ident, s.tyidents.clone())))
}

pub fn enterScope() -> P<()> {
    P::with(box |s: PState| POk(__assign!(s, { scopes: __op_concat(s.tyidents.clone(), s.scopes.clone()) }), ()))
}

pub fn leaveScope() -> P<()> {
    P::with(box |s: PState| {
        let mut ss = s.scopes.clone();
        if ss.is_empty() {
            __error!("leaveScope: already in global scope".to_string());
        } else {
            let tyids = ss.remove(0);
            POk(__assign!(s, {
                tyidents: tyids,
                scopes: ss_q,
            }), ())
        }
    })
}

pub fn getInput() -> P<InputStream> {
    P::with(box |s: PState| POk(s.clone(), s.curInput.clone()))
}

pub fn setInput(i: InputStream) -> P<()> {
    P::with(box |s: PState| POk(__assign!(s, { curInput: i }), ()))
}

pub fn getLastToken() -> P<CToken> {
    P::with(box |s: PState| POk(s.clone(), s.prevToken.clone()))
}

pub fn getSavedToken() -> P<CToken> {
    P::with(box |s: PState| POk(s.clone(), s.savedToken.clone()))
}

pub fn setLastToken(_0: CToken) -> P<()> {
    match (_0) {
        CTokEof => P::with(box |s| POk(__assign!(s, { savedToken: (prevToken(s)) }), ())),
        tok => {
            P::with(box |s| {
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
    P::with(box |s: PState| POk(__assign!(s, { savedToken: prevToken(s) }), ()))
}

pub fn getCurrentPosition() -> P<Position> {
    P::with(box |s: PState| POk(s.clone(), s.curPos.clone()))
}

// --------

pub fn rshift_monad<a: 'static, b: 'static>(a: P<a>, b: P<b>) -> P<b> {
    thenP(a, box |_| b)
}
