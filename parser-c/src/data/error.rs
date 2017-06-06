// Original file: "Error.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Typeable;
// use Language::C::Data::Node;
// use Language::C::Data::Position;

use data::position::*;
use data::node::*;

#[derive(Eq, Ord)]
pub enum ErrorLevel {
    LevelWarn,
    LevelError,
    LevelFatal,
}
pub use self::ErrorLevel::*;

pub fn isHardError() -> bool {
    ((() > LevelWarn(errorLevel)))
}

#[derive(Debug)]
pub struct ErrorInfo(pub ErrorLevel, pub Position, pub Vec<String>);


pub fn mkErrorInfo(lvl: ErrorLevel, msg: String, node: NodeInfo) -> ErrorInfo {
    ErrorInfo(lvl, (posOfNode(node)), (lines(msg)))
}

#[derive(Debug)]
pub struct CError<E: Error>(pub E);

// errors in Language.C are instance of 'Error'
pub trait Error where Self: Display {
    // obtain source location etc. of an error
    fn errorInfo(self) -> ErrorInfo;
    // wrap error in 'CError'
    fn toError(self) -> CError {
        CError(self)
    }
    // try to cast a generic 'CError' to the specific error type
    fn fromError(c: CError) -> Option<Self> {
        Some(c)
    }
    // modify the error level
    fn changeErrorLevel(self, lvl: ErrorLevel) -> Self {
        if errorLevel(self) == lvl {
            self
        } else {
            panic!("changeErrorLevel: not possible for {}", self);
        }
    }
}

//TODO
// instance Show CError where
//     show (CError e) = show e
impl Error for CError {
    fn errorInfo(self) -> ErrorInfo {
        self.0.errorInfo()
    }

    fn toError(self) -> CError {
        self
    }

    fn fromError(c: CError) -> Option<Self> {
        Some(c)
    }

    fn changeErrorLevel(self, lvl: ErrorLevel) -> Self {
        CError(self.0.changeErrorLevel(lvl))
    }
}

pub fn errorPos<E: Error>(e: E) -> Position {
    if let ErrorInfo(_, pos, _) = e.errorInfo() {
         pos
    }
}

pub fn errorLevel<E: Error>(e: E) -> ErrorLevel {
    if let ErrorInfo(lvl, _, _) = e.errorInfo() {
         lvl
    }
}

pub fn errorMsg<E: Error>(e: E) -> Vec<String> {
    if let ErrorInfo(_, _, msgs) = e.errorInfo() {
         msgs
    }
}

#[derive(Debug)]
pub struct UnsupportedFeature(pub String, pub Position);

// pub trait Error<T: UnsupportedFeature> {
    // errorInfo (UnsupportedFeature msg pos) = ErrorInfo LevelError pos (lines msg)
// instance Show UnsupportedFeature where show = showError "Unsupported Feature"


pub fn unsupportedFeature<a>(msg: String, a: a) -> UnsupportedFeature {
    UnsupportedFeature(msg, (posOf(a)))
}

pub fn unsupportedFeature_(msg: String) -> UnsupportedFeature {
    UnsupportedFeature(msg, internalPos)
}

#[derive(Debug)]
pub struct UserError(pub ErrorInfo);


pub fn userErr(msg: String) -> UserError {
    UserError((ErrorInfo(LevelError, internalPos, (lines(msg)))))
}

pub fn showError(short_msg: String) -> String {
    showErrorInfo(short_msg, errorInfo)
}

pub fn showErrorInfo(short_msg: String, ErrorInfo(level, pos, msgs): ErrorInfo) -> String {

    let header = __op_addadd(showPos(pos),
                             __op_addadd("[".to_string(),
                                         __op_addadd(show(level), "]".to_string())));

    let showMsgLines = |_0| match (_0) {
        [] => internalErr("No short message or error message provided.".to_string()),
        [x, xs] => {
                __op_addadd(indent, __op_addadd(">>> ".to_string(), __op_addadd(x, __op_addadd("\n".to_string(), unlines((__map!((indent(__op_addadd)), xs)))))))
            }
    };

    __op_addadd(header,
                showMsgLines((__op_concat(if null(short_msg) { msgs } else { short_msg }, msgs))))
}

pub fn internalErrPrefix() -> String {
    unlines(vec!["Language.C : Internal Error".to_string(),
                 __op_addadd("This is propably a bug, and should be reported at ".to_string(),
                             "http://www.sivity.net/projects/language.c/newticket".to_string())])
}

pub fn internalErr<a>(msg: String) -> a {
    __error!(__op_addadd(internalErrPrefix,
                         __op_addadd("\n".to_string(),
                                     __op_addadd(indentLines(msg), "\n".to_string()))));
}

pub fn indent() -> String {
    "  ".to_string()
}

pub fn indentLines() -> String {
    unlines(__map!((indent(__op_addadd)), lines))
}
