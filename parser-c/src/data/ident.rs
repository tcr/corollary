// Original file: "Ident.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Char;
// use Language::C::Data::Position;
// use Language::C::Data::Node;
// use Language::C::Data::Name;
// use Name;
// use Data::Generics;

use data::position::*;
use data::node::*;
use data::name::Name;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum SUERef {
    AnonymousRef(Name),
    NamedRef(Ident),
}
pub use self::SUERef::*;

pub fn isAnonymousRef(_0: SUERef) -> bool {
    match (_0) {
        AnonymousRef(_) => true,
        _ => false,
    }
}

#[derive(Clone, Debug, PartialOrd)]
pub struct Ident(pub String, pub isize, pub NodeInfo);

// the definition of the equality allows identifiers to be equal that are
// defined at different source text positions, and aims at speeding up the
// equality test, by comparing the lexemes only if the two numbers are equal
impl PartialEq for Ident {
    fn eq(&self, &Ident(ref s_, ref h_, _): &Self) -> bool {
        if let &Ident(ref s, ref h, _) = self {
            (h == h_) && (s == s_)
        } else { unreachable!() }
    }
}

// -- this does *not* follow the alphanumerical ordering of the lexemes
// --
// instance Ord Ident where
//   compare (Ident s h _) (Ident s' h' _) = compare (h, s) (h', s')

// -- identifiers are attributed
impl CNode for Ident {
    fn nodeInfo(self) -> NodeInfo {
        if let Ident(_, _, at) = self {
            at
        } else { unreachable!() }
    }
}
// instance Pos Ident where
//   posOf = posOfNode . nodeInfo

pub fn quad(_0: String) -> isize {
    let c: Vec<char> = _0.chars().collect();
    match (_0) {
    // [c1, c2, c3, c4, s..]
    if c.len() > 3 {
        let s: String = c[4..].to_vec().into_iter().collect();
        return ((__mod(((ord(c[3]) *
                    (bits21 + (ord(c[2]) * (bits14 + (ord(c[1]) * (bits7 + ord(c[0])))))))),
                bits28)) + (__mod(quad(s), bits28)))
    }
    // [c1, c2, c3]
    if c.len() == 3 {
        return (ord(c[2]) * (bits14 + (ord(c[1]) * (bits7 + ord(c[2])))));
    }
    // [c1, c2]
    if c.len() == 2 { 
        return (ord(c[1]) * (bits7 + ord(c[0])));
    }
    // [c1]
    if c.len() == 1 {
        return ord(c[0]);
    }
    // []
    return 0;
}

pub fn bits7() -> isize {
    __op_power(2, (7))
}

pub fn bits14() -> isize {
    __op_power(2, (14))
}

pub fn bits21() -> isize {
    __op_power(2, (21))
}

pub fn bits28() -> isize {
    __op_power(2, (28))
}

pub fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
    Ident(s, (quad(s)), (mkNodeInfo_q(pos, (pos, length(s)), name)))
}

pub fn internalIdent(s: String) -> Ident {
    Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
}

pub fn internalIdentAt(pos: Position, s: String) -> Ident {
    Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
}

pub fn builtinIdent(s: String) -> Ident {
    Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos())))
}

pub fn isInternalIdent(Ident(_, _, nodeinfo): Ident) -> bool {
    isInternalPos((posOfNode(nodeinfo)))
}

pub fn identToString(Ident(s, _, _): Ident) -> String {
    s
}

pub fn sueRefToString(_0: SUERef) -> String {
    match (_0) {
        AnonymousRef(_) => "".to_string(),
        NamedRef(ident) => identToString(ident),
    }
}

pub fn dumpIdent(ide: Ident) -> String {
    __op_addadd(identToString(ide),
                __op_addadd(" at ".to_string(), show((ide.nodeInfo()))))
}
