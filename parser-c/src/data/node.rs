// Original file: "Node.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::Position;
// use Language::C::Data::Name;
// use Name;
// use Data::Generics;

use data::name::Name;
use data::position::{Position, PosLength};

#[derive(Clone, Debug, Eq, Ord)]
pub enum NodeInfo {
    OnlyPos(Position, PosLength),
    NodeInfo(Position, PosLength, Name),
}
pub use self::NodeInfo::*;

pub fn lengthOfNode(ni: NodeInfo) -> Option<isize> {

    let computeLength = |firstPos, (lastPos, lastTokLen)| if lastTokLen < 0 {
        None
    } else {
        Some(posOffset(lastPos) + lastTokLen - posOffset(pos))
    };

    let len = match ni {
        NodeInfo(firstPos, lastTok, _) => computeLength(firstPos, lastTok),
        OnlyPos(firstPos, lastTok) => computeLength(firstPos, lastTok),
    };

    len
}

pub fn getLastTokenPos(_0: NodeInfo) -> PosLength {
    match (_0) {
        NodeInfo(_, lastTok, _) => lastTok,
        OnlyPos(_, lastTok) => lastTok,
    }
}

pub fn nameOfNode(_0: NodeInfo) -> Option<Name> {
    match (_0) {
        OnlyPos(_, _) => None,
        NodeInfo(_, _, name) => None,
    }
}

pub fn posOfNode(ni: NodeInfo) -> Position {
    match ni {
        OnlyPos(pos, _) => pos,
        NodeInfo(pos, _, _) => pos,
    }
}

pub fn fileOfNode() -> Option<FilePath> {

    fmap(posFile, justIf(isSourcePos, posOfNode(nodeInfo)))
}

pub fn eqByName<a>(obj1: a, obj2: a) -> bool {
    ((nodeInfo(obj1)) == (nodeInfo(obj2)))
}

pub fn internalNode() -> NodeInfo {
    undefNode
}

pub fn undefNode() -> NodeInfo {
    OnlyPos(nopos, (nopos, -(1)))
}

pub fn mkNodeInfoOnlyPos(pos: Position) -> NodeInfo {
    OnlyPos(pos, (nopos, -(1)))
}

pub fn mkNodeInfoPosLen() -> NodeInfo {
    OnlyPos
}

pub fn mkNodeInfo(pos: Position, name: Name) -> NodeInfo {
    NodeInfo(pos, (nopos, -(1)), name)
}

pub fn mkNodeInfo_q(pos: Position, lasttok: PosLength, name: Name) -> NodeInfo {
    NodeInfo(pos, lasttok, name)
}
