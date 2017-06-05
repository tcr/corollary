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

#[derive(Clone, Debug, Eq, Ord)]
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

#[derive(Clone, Debug)]
pub struct Ident(pub String, pub isize, pub NodeInfo);


pub fn quad(_0: String) -> isize {
    match (_0) {
        [c1, [c2, [c3, [c4, s]]]] => {
            ((__mod(((ord(c4) *
                      (bits21 + (ord(c3) * (bits14 + (ord(c2) * (bits7 + ord(c1)))))))),
                    bits28)) + (__mod(quad(s), bits28)))
        }
        [c1, [c2, [c3, []]]] => (ord(c3) * (bits14 + (ord(c2) * (bits7 + ord(c1))))),
        [c1, [c2, []]] => (ord(c2) * (bits7 + ord(c1))),
        [c1, []] => ord(c1),
        [] => 0,
    }
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
    Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
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
                __op_addadd(" at ".to_string(), show((nodeInfo(ide)))))
}
