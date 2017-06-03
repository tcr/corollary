#[macro_use] use corollary_support::*;

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
    NamedRef(Ident)
}
pub use self::SUERef::*;

#[derive(Clone, Debug)]
pub struct Ident(String, isize, NodeInfo);


pub fn bits14<a>() -> isize {
    __op_power(2, (14))
}

pub fn bits21<a>() -> isize {
    __op_power(2, (21))
}

pub fn bits28<a>() -> isize {
    __op_power(2, (28))
}

pub fn bits7<a>() -> isize {
    __op_power(2, (7))
}

pub fn builtinIdent<a>(s: String) -> Ident {
    Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
}

pub fn dumpIdent<a>(ide: Ident) -> String {
    __op_addadd(identToString(ide), __op_addadd(" at ".to_string(), show((nodeInfo(ide)))))
}

pub fn identToString<a>(Ident(s, _, _): Ident) -> String {
    s
}

pub fn internalIdent<a>(s: String) -> Ident {
    Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
}

pub fn internalIdentAt<a>(pos: Position, s: String) -> Ident {
    Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
}

pub fn isAnonymousRef<a>(_0: SUERef) -> bool {
    match (_0) {
        AnonymousRef(_) => {
            true
        },
        _ => {
            false
        },
    }
}

pub fn isInternalIdent<a>(Ident(_, _, nodeinfo): Ident) -> bool {
    isInternalPos((posOfNode(nodeinfo)))
}

pub fn mkIdent<a>(pos: Position, s: String, name: Name) -> Ident {
    Ident(s, (quad(s)), (mkNodeInfo_q(pos, (pos, length(s)), name)))
}

pub fn quad<a>(_0: String) -> isize {
    match (_0) {
        [c1, [c2, [c3, [c4, s]]]] => {
            ((__mod(((ord(c4) * (bits21 + (ord(c3) * (bits14 + (ord(c2) * (bits7 + ord(c1)))))))), bits28)) + (__mod(quad(s), bits28)))
        },
        [c1, [c2, [c3, []]]] => {
            (ord(c3) * (bits14 + (ord(c2) * (bits7 + ord(c1)))))
        },
        [c1, [c2, []]] => {
            (ord(c2) * (bits7 + ord(c1)))
        },
        [c1, []] => {
            ord(c1)
        },
        [] => {
            0
        },
    }
}



