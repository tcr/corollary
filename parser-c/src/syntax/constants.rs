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

pub fn showCharConst(c: Char) -> ShowS {
    sQuote(escapeCChar(c))
}

pub fn _showWideFlag(flag: bool) -> ShowS {
    if flag {     
showString("L".to_string())} else {
id
    }
}

pub fn getCChar(_0: CChar) -> String {
    match (_0) {
        CChar(c, _) => {
            vec![c]
        },
        CChars(cs, _) => {
            vec![c]
        },
    }
}

pub fn getCCharAsInt(_0: CChar) -> Integer {
    match (_0) {
        CChar(c, _) => {
            fromIntegral((fromEnum(c)))
        },
        CChars(_cs, _) => {
            fromIntegral((fromEnum(c)))
        },
    }
}

pub fn isWideChar(_0: CChar) -> bool {
    match (_0) {
        CChar(_, wideFlag) => {
            wideFlag
        },
        CChars(_, wideFlag) => {
            wideFlag
        },
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


pub fn readCInteger(repr: CIntRepr, __str: String) -> Either<String, CInteger> {

    let readNum = match repr {
            DecRepr => {
                readDec
            },
            HexRepr => {
                readHex
            },
            OctalRepr => {
                readOct
            },
        };

    let mkCInt = |n, suffix| {
        either(Left, (Right(CInteger(n, repr))), readSuffix(suffix))
    };

    let readSuffix = parseFlags(noFlags);

    let parseFlags = |_0, _1| {
        match (_0, _1) {
            (flags, []) => {
                Right(flags)
            },
            (flags, ['l', ['l', fs]]) => {
                Right(flags)
            },
            (flags, ['L', ['L', fs]]) => {
                Right(flags)
            },
            (flags, [f, fs]) => {
                Right(flags)
            },
        }
    };

    match readNum(__str) {
        [(n, suffix)] => {
            mkCInt(n, suffix)
        },
        parseFailed => {
            Left(__op_addadd("Bad Integer literal: ".to_string(), show(parseFailed)))
        },
    }
}

pub fn getCInteger(CInteger(i, _, _): CInteger) -> Integer {
    i
}

pub fn cInteger(i: Integer) -> CInteger {
    CInteger(i, DecRepr, noFlags)
}

#[derive(Clone, Debug, Eq, Ord)]
pub struct CFloat(String);


pub fn cFloat() -> CFloat {
    CFloat(show)
}

pub fn readCFloat() -> CFloat {
    CFloat
}

#[derive(Clone, Debug, Eq, Ord)]
pub struct ClangCVersion(String);


pub fn readClangCVersion() -> ClangCVersion {
    ClangCVersion
}

#[derive(Clone, Debug, Eq, Ord)]
pub struct CString(String, bool);


pub fn cString(__str: String) -> CString {
    CString(__str, false)
}

pub fn cString_w(__str: String) -> CString {
    CString(__str, true)
}

pub fn getCString(CString(__str, _): CString) -> String {
    __str
}

pub fn isWideString(CString(_, wideflag): CString) -> bool {
    wideflag
}

pub fn concatCStrings(cs: Vec<CString>) -> CString {
    CString((concatMap(getCString, cs)), (any(isWideString, cs)))
}

pub fn showStringLit() -> ShowS {

    dQuote(concatMap(showStringChar))
}

pub fn isAsciiSourceChar(c: Char) -> bool {
    (isAscii(c) && isPrint(c))
}

pub fn isCChar(_0: Char) -> bool {
    match (_0) {
        '\\' => {
            false
        },
        '\'' => {
            false
        },
        '\n' => {
            false
        },
        c => {
            false
        },
    }
}

pub fn escapeCChar(_0: Char) -> String {
    match (_0) {
        '\'' => {
            "\\\'".to_string()
        },
        c => {
            "\\\'".to_string()
        },
    }
}

pub fn isSChar(_0: Char) -> bool {
    match (_0) {
        '\\' => {
            false
        },
        '\"' => {
            false
        },
        '\n' => {
            false
        },
        c => {
            false
        },
    }
}

pub fn showOct_q(i: isize) -> String {

    let s = showOct(i, "".to_string());

    __op_addadd(replicate(((3 - length(s))), '0'), s)
}

pub fn escapeChar(_0: Char) -> String {
    match (_0) {
        '\\' => {
            "\\\\".to_string()
        },
        '\u{7}' => {
            "\\\\".to_string()
        },
        '\u{8}' => {
            "\\\\".to_string()
        },
        '\u{1b}' => {
            "\\\\".to_string()
        },
        '\u{c}' => {
            "\\\\".to_string()
        },
        '\n' => {
            "\\\\".to_string()
        },
        '\r' => {
            "\\\\".to_string()
        },
        '\t' => {
            "\\\\".to_string()
        },
        '\u{b}' => {
            "\\\\".to_string()
        },
        c => {
            "\\\\".to_string()
        },
    }
}

pub fn unescapeChar(_0: String) -> (Char, String) {
    match (_0) {
        ['\\', [c, cs]] => {
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
        [c, cs] => {
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
        [] => {
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

pub fn readOct_q(s: ReadS<isize>) -> ReadS<isize> {

    let octStr = takeWhile(isOctDigit, take(3, s));

    let rest = drop((length(octStr)), s);

    __map!((|(i, cs)| { (i, __op_addadd(cs, rest)) }), (readOct(octStr)))
}

pub fn unescapeString(_0: String) -> String {
    match (_0) {
        [] => {
            vec![]
        },
        cs => {
            vec![]
        },
    }
}

pub fn sQuote(s: String, t: ShowS) -> ShowS {
    __op_addadd("\'".to_string(), __op_addadd(s, __op_addadd("\'".to_string(), t)))
}

pub fn dQuote(s: String, t: ShowS) -> ShowS {
    __op_addadd((__op_concat('\"', s)), __op_addadd("\"".to_string(), t))
}

pub fn head_q<a>(_0: String, _1: Vec<a>) -> a {
    match (_0, _1) {
        (err, []) => {
            __error!(err)
        },
        (_, [x, _]) => {
            __error!(err)
        },
    }
}

#[derive(Clone, Debug, Eq, Ord)]
pub struct Flags<f>(Integer);


pub fn noFlags() -> Flags<f> {
    Flags(0)
}

pub fn setFlag(flag: f, Flags(k): Flags<f>) -> Flags<f> {
    Flags(setBit(k, fromEnum(flag)))
}

pub fn clearFlag(flag: f, Flags(k): Flags<f>) -> Flags<f> {
    Flags(clearBit(k, fromEnum(flag)))
}

pub fn testFlag(flag: f, Flags(k): Flags<f>) -> bool {
    testBit(k, fromEnum(flag))
}



