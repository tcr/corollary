use haskell_support::*;

#[derive(Clone, Debug)]
pub enum NodeInfo {
    OnlyPos(Position, PosLength),
    NodeInfo(Position, PosLength, Name)
}
pub use self::NodeInfo::*;

pub fn eqByName(obj1: a, obj2: a) -> bool {
    ((nodeInfo(obj1)) == (nodeInfo(obj2)))
}

pub fn fileOfNode() -> Option<FilePath> {
    fmap(posFile, justIf(isSourcePos, posOfNode(nodeInfo)))
}

pub fn getLastTokenPos(__0: NodeInfo) -> PosLength {
    match (__0) {
        NodeInfo(_, lastTok, _) => {
            lastTok
        },
        OnlyPos(_, lastTok) => {
            lastTok
        },
    }
}

pub fn internalNode() -> NodeInfo {
    undefNode
}

pub fn isUndefNode(_: NodeInfo) -> bool {
    false
}

pub fn lengthOfNode(ni: NodeInfo) -> Option<isize> {
    len
}

pub fn mkNodeInfo(pos: Position, name: Name) -> NodeInfo {
    NodeInfo(pos, (nopos, -(1)), name)
}

pub fn mkNodeInfoOnlyPos(pos: Position) -> NodeInfo {
    OnlyPos(pos, (nopos, -(1)))
}

pub fn mkNodeInfoPosLen() -> NodeInfo {
    OnlyPos
}

pub fn mkNodeInfo_q(pos: Position, lasttok: PosLength, name: Name) -> NodeInfo {
    NodeInfo(pos, lasttok, name)
}

pub fn nameOfNode(__0: NodeInfo) -> Option<Name> {
    match (__0) {
        OnlyPos(_, _) => {
            None
        },
        NodeInfo(_, _, name) => {
            Some(name)
        },
    }
}

pub fn posOfNode(ni: NodeInfo) -> Position {
    match ni {
        OnlyPos(pos, _) => {
            pos
        },
        NodeInfo(pos, _, _) => {
            pos
        },
    }
}

pub fn undefNode() -> NodeInfo {
    OnlyPos(nopos, (nopos, -(1)))
}

