// Original file: "Constants.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Bits;
// use Data::char;
// use Numeric;
// use showOct;
// use Data::Generics;

use num::ToPrimitive;
use std::marker::PhantomData;
use data::error::Error;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CChar {
    CChar(char, bool),
    CChars(Vec<char>, bool),
}
pub use self::CChar::*;

pub fn showCharConst(c: char) -> ShowS {
    sQuote(escapeCChar(c))
}

pub fn _showWideFlag(flag: bool) -> ShowS {
    if flag {
        showString("L".to_string())
    } else {
        showString("".to_string())
    }
}

pub fn getCChar(_0: CChar) -> String {
    match (_0) {
        CChar(c, _) => vec![c],
        CChars(cs, _) => cs,
    }
}

pub fn getCCharAsInt(_0: CChar) -> isize {
    match (_0) {
        CChar(c, _) => fromIntegral((fromEnum(c))),
        CChars(_cs, _) => {
            __error!("integer value of multi-character character constants is implementation defined".to_string())
        }
    }
}

pub fn isWideChar(_0: CChar) -> bool {
    match (_0) {
        CChar(_, wideFlag) => wideFlag,
        CChars(_, wideFlag) => wideFlag,
    }
}

pub fn cChar(c: char) -> CChar {
    CChar(c, false)
}

pub fn cChar_w(c: char) -> CChar {
    CChar(c, true)
}

pub fn cChars() -> CChar {
    CChars
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CIntRepr {
    DecRepr,
    HexRepr,
    OctalRepr,
}
pub use self::CIntRepr::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CIntFlag {
    FlagUnsigned,
    FlagLong,
    FlagLongLong,
    FlagImag,
}
pub use self::CIntFlag::*;

impl ToPrimitive for CIntFlag {
    fn to_i64(&self) -> Option<i64> {
        Some(match *self {
            FlagUnsigned => 0,
            FlagLong => 1,
            FlagLongLong => 2,
            FlagImag => 3,
        })
    }
    fn to_u64(&self) -> Option<u64> {
        self.to_i64().map(|x| x as u64)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CInteger(pub isize, pub CIntRepr, pub Flags<CIntFlag>);


pub fn readCInteger(repr: CIntRepr, __str: String) -> Either<String, CInteger> {

    let readNum = match repr {
        DecRepr => readDec,
        HexRepr => readHex,
        OctalRepr => readOct,
    };

    let mkCInt = |n, suffix| {
        match parseFlags(noFlags, suffix) {
            s @ Left(..) => s,
            // TODO not sure this is right
            Right(value) => CInteger(n, repr)
        }
    };

    fn parseFlags<T: ToPrimitive>(_0: Flags<T>, _1: String) -> Either<String, Flags<T>> {
        match (_0, __boxed_chars(_1)) {
            (flags, box []) => Right(flags),
            (flags, box ['l', 'l', fs..]) => parseFlags((setFlag(FlagLongLong, flags)), fs),
            (flags, box ['L', 'L', fs..]) => parseFlags((setFlag(FlagLongLong, flags)), fs),
            (flags, box [f, fs..]) => {
                let go1 = |flag| parseFlags((setFlag(flag, flags)), fs);

                match f {
                    'l' => go1(FlagLong),
                    'L' => go1(FlagLong),
                    'u' => go1(FlagUnsigned),
                    'U' => go1(FlagUnsigned),
                    'i' => go1(FlagImag),
                    'I' => go1(FlagImag),
                    'j' => go1(FlagImag),
                    'J' => go1(FlagImag),
                    _ => Left(__op_addadd("Unexpected flag ".to_string(), show(f))),
                }
            }
        }
    }

    match readNum(__str) {
        [(n, suffix)] => mkCInt(n, suffix),
        parseFailed => Left(__op_addadd("Bad Integer literal: ".to_string(), show(parseFailed))),
    }
}

pub fn getCInteger(CInteger(i, _, _): CInteger) -> isize {
    i
}

pub fn cInteger(i: isize) -> CInteger {
    CInteger(i, DecRepr, noFlags)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CFloat(pub String);


pub fn cFloat() -> CFloat {
    CFloat(show)
}

pub fn readCFloat() -> CFloat {
    CFloat
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ClangCVersion(pub String);


pub fn readClangCVersion() -> ClangCVersion {
    ClangCVersion
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CString(pub String, pub bool);


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
    CString((__concatMap!(getCString, cs)), (any(isWideString, cs)))
}

pub fn showStringLit(s: String) -> ShowS {
    let showStringChar = |c| if isSChar(c) {
        c
    } else if c == '"' {
        "\\\"".to_string()
    } else {
        escapeChar(c)
    };
    dQuote(__concatMap!(showStringChar, s))
}

pub fn isAsciiSourceChar(c: char) -> bool {
    (isAscii(c) && isPrint(c))
}

pub fn isCChar(_0: char) -> bool {
    match (_0) {
        '\\' => false,
        '\'' => false,
        '\n' => false,
        c => isAsciiSourceChar(c),
    }
}

pub fn escapeCChar(_0: char) -> String {
    match (_0) {
        '\'' => "\\\'".to_string(),
        c => {
            if isSChar(c) {
                c.to_string()
            } else {
                escapeChar(c)
            }
        }
    }
}

pub fn isSChar(_0: char) -> bool {
    match (_0) {
        '\\' => false,
        '\"' => false,
        '\n' => false,
        c => isAsciiSourceChar(c),
    }
}

pub fn showOct_q(i: isize) -> String {

    let s = showOct(i).show_s("".to_string());

    __op_addadd(replicate(((3 - length(s))), '0'), s)
}

pub fn escapeChar(_0: char) -> String {
    match (_0) {
        '\\' => "\\\\".to_string(),
        '\u{7}' => "\\a".to_string(),
        '\u{8}' => "\\b".to_string(),
        '\u{1b}' => "\\e".to_string(),
        '\u{c}' => "\\f".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        '\u{b}' => "\\v".to_string(),
        c => {
            if ord(c) < 512 {
                format!("\\{}", showOct_q(ord(c)))
            } else {
                format!("\\x{}", showHex(ord(c)).show_s("".to_string()))
            }
        }
    }
}

pub fn unescapeChar(_0: String) -> (char, String) {
    match __boxed_chars(_0) {
        box ['\\', c, cs..] => {
            match c {
                'n' => ('\n', cs),
                't' => ('\t', cs),
                'v' => ('\u{b}', cs),
                'b' => ('\u{8}', cs),
                'r' => ('\r', cs),
                'f' => ('\u{c}', cs),
                'a' => ('\u{7}', cs),
                'e' => ('\u{1b}', cs),
                'E' => ('\u{1b}', cs),
                '\\' => ('\\', cs),
                '?' => ('?', cs),
                '\'' => ('\'', cs),
                '\"' => ('\"', cs),
                'x' => {
                    match head_q("bad escape sequence".to_string(), (readHex(cs))) {
                        (i, cs_q) => (i, cs_q),
                    }
                }
                _ => {
                    match head_q("bad escape sequence".to_string(),
                                 (readOct_q((__op_concat(c, cs))))) {
                        (i, cs_q) => (i, cs_q),
                    }
                }
            }
        }
        box [c, cs..] => (c, cs),
        box [] => __error!("unescape char: empty string".to_string()),
    }
}

pub fn readOct_q(s: ReadS<isize>) -> ReadS<isize> {

    let octStr = takeWhile(isOctDigit, take(3, s));

    let rest = drop((length(octStr)), s);

    __map!((|(i, cs)| (i, __op_addadd(cs, rest))), (readOct(octStr)))
}

pub fn unescapeString(_0: String) -> String {
    match (_0) {
        [] => vec![],
        cs => {
            match unescapeChar(cs) {
                (c, cs_q) => __op_concat(c, unescapeString(cs_q)),
            }
        }
    }
}

pub fn sQuote(s: String, t: ShowS) -> ShowS {
    __op_addadd("\'".to_string(),
                __op_addadd(s, __op_addadd("\'".to_string(), t)))
}

pub fn dQuote(s: String, t: ShowS) -> ShowS {
    __op_addadd(__op_concat('\"', s), __op_addadd("\"".to_string(), t))
}

pub fn head_q<a>(_0: String, _1: Vec<a>) -> a {
    match (_0, _1.into_boxed_slice()) {
        (err, box []) => __error!(err),
        (_, box [x, _]) => x,
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Flags<F: ToPrimitive>{
    flags: isize,
    _phantom: PhantomData<F>,
}

impl<F: ToPrimitive> Flags<F> {
    fn new(flags: isize) -> Self {
        Flags {
            flags,
            _phantom: PhantomData::new(),
        }
    }
}


pub fn noFlags<f: ToPrimitive>() -> Flags<f> {
    Flags::new(0)
}

pub fn setFlag<f: ToPrimitive>(flag: f, Flags { flags: k, .. }: Flags<f>) -> Flags<f> {
    Flags::new(setBit(k, flag.to_isize().unwrap()))
}

pub fn clearFlag<f: ToPrimitive>(flag: f, Flags { flags: k, .. }: Flags<f>) -> Flags<f> {
    Flags::new(clearBit(k, flag.to_isize().unwrap()))
}

pub fn testFlag<f: ToPrimitive>(flag: f, Flags { flags: k, .. }: Flags<f>) -> bool {
    testBit(k, flag.to_isize().unwrap())
}
