// Original file: "Constants.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Bits;
// use Data::Char;
// use Numeric;
// use showOct;
// use Data::Generics;

#[derive(Clone, Debug, Eq, Ord)]
pub enum CChar {
    CChar(Char, bool),
    CChars(Vec<Char>, bool)
}
pub use self::CChar::*;

#[derive(Bounded, Clone, Debug, Enum, Eq, Ord)]
pub enum CIntRepr {
    DecRepr,
    HexRepr,
    OctalRepr
}
pub use self::CIntRepr::*;

#[derive(Bounded, Clone, Debug, Enum, Eq, Ord)]
pub enum CIntFlag {
    FlagUnsigned,
    FlagLong,
    FlagLongLong,
    FlagImag
}
pub use self::CIntFlag::*;

#[derive(Clone, Debug, Eq, Ord)]
pub struct CInteger(Integer, CIntRepr, Flags<CIntFlag>);


#[derive(Clone, Debug, Eq, Ord)]
pub struct CFloat(String);


#[derive(Clone, Debug, Eq, Ord)]
pub struct ClangCVersion(String);


#[derive(Clone, Debug, Eq, Ord)]
pub struct CString(String, bool);


#[derive(Clone, Debug, Eq, Ord)]
pub struct Flags<f>(Integer);


pub fn _showWideFlag(flag: bool) -> ShowS {
    if flag {     
showString("L".to_string())} else {
id
    }
}

pub fn cChar(c: Char) -> CChar {
    CChar(c, false)
}

pub fn cChar_w(c: Char) -> CChar {
    CChar(c, true)
}

pub fn cChars() -> CChar {
    CChars
}

pub fn cFloat() -> CFloat {
    CFloat(show)
}

pub fn cInteger(i: Integer) -> CInteger {
    CInteger(i, DecRepr, noFlags)
}

pub fn cString(__str: String) -> CString {
    CString(__str, false)
}

pub fn cString_w(__str: String) -> CString {
    CString(__str, true)
}

pub fn clearFlag(flag: f, Flags(k): Flags<f>) -> Flags<f> {
    Flags(clearBit(k, fromEnum(flag)))
}

pub fn concatCStrings(cs: Vec<CString>) -> CString {
    CString((concatMap(getCString, cs)), (any(isWideString, cs)))
}

pub fn dQuote(s: String, t: ShowS) -> ShowS {
    __op_addadd((__op_concat('\"', s)), __op_addadd("\"".to_string(), t))
}

pub fn escapeCChar(_0: Char) -> String {
    match (_0) {
        _0 => {
            "\\\'".to_string()
        },
        _0 => {
            "\\\'".to_string()
        },
    }
}

pub fn escapeChar(_0: Char) -> String {
    match (_0) {
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
        _0 => {
            "\\\\".to_string()
        },
    }
}

pub fn getCChar(_0: CChar) -> String {
    match (_0) {
        _0 => {
            vec![c]
        },
        _0 => {
            vec![c]
        },
    }
}

pub fn getCCharAsInt(_0: CChar) -> Integer {
    match (_0) {
        _0 => {
            fromIntegral((fromEnum(c)))
        },
        _0 => {
            fromIntegral((fromEnum(c)))
        },
    }
}

pub fn getCInteger(CInteger(i, _, _): CInteger) -> Integer {
    i
}

pub fn getCString(CString(__str, _): CString) -> String {
    __str
}

pub fn head_q<a>(_0: String, _1: Vec<a>) -> a {
    match (_0, _1) {
        (_0, _1) => {
            __error!(err)
        },
        (_0, _1) => {
            __error!(err)
        },
    }
}

pub fn isAsciiSourceChar(c: Char) -> bool {
    (isAscii(c) && isPrint(c))
}

pub fn isCChar(_0: Char) -> bool {
    match (_0) {
        _0 => {
            false
        },
        _0 => {
            false
        },
        _0 => {
            false
        },
        _0 => {
            false
        },
    }
}

pub fn isSChar(_0: Char) -> bool {
    match (_0) {
        _0 => {
            false
        },
        _0 => {
            false
        },
        _0 => {
            false
        },
        _0 => {
            false
        },
    }
}

pub fn isWideChar(_0: CChar) -> bool {
    match (_0) {
        _0 => {
            wideFlag
        },
        _0 => {
            wideFlag
        },
    }
}

pub fn isWideString(CString(_, wideflag): CString) -> bool {
    wideflag
}

pub fn noFlags() -> Flags<f> {
    Flags(0)
}

pub fn readCFloat() -> CFloat {
    CFloat
}

pub fn readCInteger(repr: CIntRepr, __str: String) -> Either<String, CInteger> {
    match readNum(__str) {
        [(n, suffix)] => {
            mkCInt(n, suffix)
        },
        parseFailed => {
            Left(__op_addadd("Bad Integer literal: ".to_string(), show(parseFailed)))
        },
    }
}

pub fn readClangCVersion() -> ClangCVersion {
    ClangCVersion
}

pub fn readOct_q(s: ReadS<isize>) -> ReadS<isize> {
    __map!((|(i, cs)| { (i, __op_addadd(cs, rest)) }), (readOct(octStr)))
}

pub fn sQuote(s: String, t: ShowS) -> ShowS {
    __op_addadd("\'".to_string(), __op_addadd(s, __op_addadd("\'".to_string(), t)))
}

pub fn setFlag(flag: f, Flags(k): Flags<f>) -> Flags<f> {
    Flags(setBit(k, fromEnum(flag)))
}

pub fn showCharConst(c: Char) -> ShowS {
    sQuote(escapeCChar(c))
}

pub fn showOct_q(i: isize) -> String {
    __op_addadd(replicate(((3 - length(s))), '0'), s)
}

pub fn showStringLit() -> ShowS {
    dQuote(concatMap(showStringChar))
}

pub fn testFlag(flag: f, Flags(k): Flags<f>) -> bool {
    testBit(k, fromEnum(flag))
}

pub fn unescapeChar(_0: String) -> (Char, String) {
    match (_0) {
        _0 => {
            match c {
                'n' => {
                    ('\n', cs)
                },
                't' => {
                    ('\t', cs)
                },
                'v' => {
                    ('\u{b}', cs)
                },
                'b' => {
                    ('\u{8}', cs)
                },
                'r' => {
                    ('\r', cs)
                },
                'f' => {
                    ('\u{c}', cs)
                },
                'a' => {
                    ('\u{7}', cs)
                },
                'e' => {
                    ('\u{1b}', cs)
                },
                'E' => {
                    ('\u{1b}', cs)
                },
                '\\' => {
                    ('\\', cs)
                },
                '?' => {
                    ('?', cs)
                },
                '\'' => {
                    ('\'', cs)
                },
                '\"' => {
                    ('\"', cs)
                },
                'x' => {
                    match head_q("bad escape sequence".to_string(), (readHex(cs))) {
                        (i, cs_q) => {
                            (toEnum(i), cs_q)
                        },
                    }
                },
                _ => {
                    match head_q("bad escape sequence".to_string(), (readOct_q((__op_concat(c, cs))))) {
                        (i, cs_q) => {
                            (toEnum(i), cs_q)
                        },
                    }
                },
            }
        },
        _0 => {
            match c {
                'n' => {
                    ('\n', cs)
                },
                't' => {
                    ('\t', cs)
                },
                'v' => {
                    ('\u{b}', cs)
                },
                'b' => {
                    ('\u{8}', cs)
                },
                'r' => {
                    ('\r', cs)
                },
                'f' => {
                    ('\u{c}', cs)
                },
                'a' => {
                    ('\u{7}', cs)
                },
                'e' => {
                    ('\u{1b}', cs)
                },
                'E' => {
                    ('\u{1b}', cs)
                },
                '\\' => {
                    ('\\', cs)
                },
                '?' => {
                    ('?', cs)
                },
                '\'' => {
                    ('\'', cs)
                },
                '\"' => {
                    ('\"', cs)
                },
                'x' => {
                    match head_q("bad escape sequence".to_string(), (readHex(cs))) {
                        (i, cs_q) => {
                            (toEnum(i), cs_q)
                        },
                    }
                },
                _ => {
                    match head_q("bad escape sequence".to_string(), (readOct_q((__op_concat(c, cs))))) {
                        (i, cs_q) => {
                            (toEnum(i), cs_q)
                        },
                    }
                },
            }
        },
        _0 => {
            match c {
                'n' => {
                    ('\n', cs)
                },
                't' => {
                    ('\t', cs)
                },
                'v' => {
                    ('\u{b}', cs)
                },
                'b' => {
                    ('\u{8}', cs)
                },
                'r' => {
                    ('\r', cs)
                },
                'f' => {
                    ('\u{c}', cs)
                },
                'a' => {
                    ('\u{7}', cs)
                },
                'e' => {
                    ('\u{1b}', cs)
                },
                'E' => {
                    ('\u{1b}', cs)
                },
                '\\' => {
                    ('\\', cs)
                },
                '?' => {
                    ('?', cs)
                },
                '\'' => {
                    ('\'', cs)
                },
                '\"' => {
                    ('\"', cs)
                },
                'x' => {
                    match head_q("bad escape sequence".to_string(), (readHex(cs))) {
                        (i, cs_q) => {
                            (toEnum(i), cs_q)
                        },
                    }
                },
                _ => {
                    match head_q("bad escape sequence".to_string(), (readOct_q((__op_concat(c, cs))))) {
                        (i, cs_q) => {
                            (toEnum(i), cs_q)
                        },
                    }
                },
            }
        },
    }
}

pub fn unescapeString(_0: String) -> String {
    match (_0) {
        _0 => {
            vec![]
        },
        _0 => {
            vec![]
        },
    }
}



