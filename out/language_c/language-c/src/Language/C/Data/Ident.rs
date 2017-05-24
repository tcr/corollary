pub mod Language_C_Data_Ident {
    use haskell_support::*;
    #[derive(Clone, Debug, Eq, Ord)]
    pub enum SUERef {
        AnonymousRef<Name>,
        NamedRef<Ident>
    }
    pub use self::SUERef::*;

    #[derive(Clone, Debug)]
    struct Ident(Ident<String, isize, NodeInfo>);

    pub fn bits14() -> isize {
        ^(2, (14))
    }

    pub fn bits21() -> isize {
        ^(2, (21))
    }

    pub fn bits28() -> isize {
        ^(2, (28))
    }

    pub fn bits7() -> isize {
        ^(2, (7))
    }

    pub fn builtinIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(builtinPos)))
    }

    pub fn dumpIdent(ide: Ident) -> String {
        __op_addadd(identToString(ide), __op_addadd(" at ".to_string(), show((nodeInfo(ide)))))
    }

    pub fn identToString((Ident(s, _, _)): Ident) -> String {
        s
    }

    pub fn internalIdent(s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoOnlyPos(internalPos)))
    }

    pub fn internalIdentAt(pos: Position, s: String) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfoPosLen(pos, (pos, length(s)))))
    }

    pub fn isAnonymousRef(__0: SUERef) -> bool {
        match (__0) {
            AnonymousRef(_) => {
                true
            },
            _ => {
                false
            },
        }
    }

    pub fn isInternalIdent((Ident(_, _, nodeinfo)): Ident) -> bool {
        isInternalPos((posOfNode(nodeinfo)))
    }

    pub fn mkIdent(pos: Position, s: String, name: Name) -> Ident {
        Ident(s, (quad(s)), (mkNodeInfo_q(pos, (pos, length(s)), name)))
    }

    pub fn quad(__0: String) -> isize {
        match (__0) {
            [c1, ...[c2, ...[c3, ...[c4, ...s]]]] => {
                +((mod(((ord(c4) * +(bits21, (ord(c3) * +(bits14, (ord(c2) * +(bits7, ord(c1)))))))), bits28)), (mod(quad(s), bits28)))
            },
            [c1, ...[c2, ...[c3, ...[]]]] => {
                (ord(c3) * +(bits14, (ord(c2) * +(bits7, ord(c1)))))
            },
            [c1, ...[c2, ...[]]] => {
                (ord(c2) * +(bits7, ord(c1)))
            },
            [c1, ...[]] => {
                ord(c1)
            },
            [] => {
                0
            },
        }
    }

}

