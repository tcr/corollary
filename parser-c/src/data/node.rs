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
use data::position::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum NodeInfo {
    OnlyPos(Position, PosLength),
    NodeInfo(Position, PosLength, Name),
}
pub use self::NodeInfo::*;

pub fn lengthOfNode(ni: NodeInfo) -> Option<isize> {

    let computeLength = |pos, (lastPos, lastTokLen)| {
        if lastTokLen < 0 {
            None
        } else {
            Some(posOffset(lastPos) + lastTokLen - posOffset(pos))
        }
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

// a class for convenient access to the attributes of an attributed object
pub trait CNode {
    fn nodeInfo(self) -> NodeInfo;
}
impl CNode for NodeInfo {
    fn nodeInfo(self) -> NodeInfo { self }
}

pub fn nodeInfo<T: CNode>(n: T) -> NodeInfo {
    n.nodeInfo()
}

// impl CNode for (CNode<a>, CNode<b>) {
//     pub fn nodeInfo(self) -> NodeInfo { Either(a, b) }
//     => CNode (Either a b) where
//   nodeInfo = either nodeInfo nodeInfo

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

pub fn fileOfNode<A: CNode>(obj: A) -> Option<FilePath> {
    fn justIf<T>(predicate: bool, x: T) -> Option<T> {
        if predicate { Some(x) } else { None }
    }

    __fmap!(posFile, justIf(isSourcePos(posOfNode(obj.nodeInfo()))))
}

pub fn eqByName<A: CNode>(obj1: A, obj2: A) -> bool {
    obj1.nodeInfo() == obj2.nodeInfo()
}

pub fn internalNode() -> NodeInfo {
    undefNode()
}

pub fn undefNode() -> NodeInfo {
    OnlyPos(nopos(), (nopos(), -(1)))
}

pub fn mkNodeInfoOnlyPos(pos: Position) -> NodeInfo {
    OnlyPos(pos, (nopos(), -(1)))
}

pub fn mkNodeInfoPosLen(a: Position, b: PosLength) -> NodeInfo {
    OnlyPos(a, b)
}

pub fn mkNodeInfo(pos: Position, name: Name) -> NodeInfo {
    NodeInfo(pos, (nopos(), -(1)), name)
}

pub fn mkNodeInfo_q(pos: Position, lasttok: PosLength, name: Name) -> NodeInfo {
    NodeInfo(pos, lasttok, name)
}
