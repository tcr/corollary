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

pub fn showCharConst(c: char) -> Box<ShowS> {
    sQuote(escapeCChar(c))
}

pub fn _showWideFlag(flag: bool) -> Box<ShowS> {
    box if flag {
        showString("L".to_string())
    } else {
        showString("".to_string())
    }
}

pub fn getCChar(_0: CChar) -> String {
    match (_0) {
        CChar(c, _) => c.to_string(),
        CChars(cs, _) => cs.into_iter().collect(),
    }
}

pub fn getCCharAsInt(_0: CChar) -> isize {
    match (_0) {
        CChar(c, _) => fromIntegral(c as isize),
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

pub fn cChars(a: String, b: bool) -> CChar {
    CChars(a.chars().collect(), b)
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

    let readNum: Box<Fn(String) -> Box<ReadS<isize>>> = match repr {
        DecRepr => box |x| box readDec(x),
        HexRepr => box |x| box readHex(x),
        OctalRepr => box |x| box readOct(x),
    };

    fn readSuffix(input: String) -> Either<String, Flags<CIntFlag>> {
        parseFlags(noFlags(), input)
    }

    fn mkCInt(n: isize, suffix: String) -> Either<String, CInteger> {
        match readSuffix(suffix) {
            Left(s) => Left(s),
            Right(s) => Right(CInteger(n, repr, s))
        }
    }

    fn parseFlags(flags: Flags<CIntFlag>, _1: String) -> Either<String, Flags<CIntFlag>> {
        // []
        if _1.len() == 0 {
            return Right(flags);
        }
        // ['l', 'l', fs...]
        if _1.starts_with("ll") {
            let fs: String = _1.chars().skip(2).collect();
            return parseFlags((setFlag(FlagLongLong, flags)), fs);
        }
        // ['L', 'L', fs...]
        if _1.starts_with("LL") {
            let fs: String = _1.chars().skip(2).collect();
            return parseFlags((setFlag(FlagLongLong, flags)), fs);
        }
        // [f, fs..]
        let f = _1.chars().next().unwrap();
        let fs: String = _1.chars().skip(2).collect();
        
        let go1 = |flag: CIntFlag| {
            parseFlags((setFlag(flag, flags)), fs)
        };

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

    let s = readNum(__str).read_s();
    if s.len() == 1 {
        if let (n, suffix) = s[0] {
            mkCInt(n, suffix)
        } else { unreachable!() }
    } else {
        Left(format!("Bad Integer literal: {:?}", s))
    }
}

pub fn getCInteger(CInteger(i, _, _): CInteger) -> isize {
    i
}

pub fn cInteger(i: isize) -> CInteger {
    CInteger(i, DecRepr, noFlags())
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CFloat(pub String);


pub fn cFloat(input: f32) -> CFloat {
    CFloat(show(input))
}

pub fn readCFloat(input: String) -> CFloat {
    CFloat(input)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ClangCVersion(pub String);


pub fn readClangCVersion(input: String) -> ClangCVersion {
    ClangCVersion(input)
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
    CString(cs.into_iter()
        .map(getCString)
        .collect::<Vec<_>>().join(""), (any(isWideString, cs)))
}

pub fn showStringLit(s: String) -> Box<ShowS> {
    dQuote(s.chars().map(|c| {
        if isSChar(c) {
            format!("{}", c)
        } else if c == '"' {
            "\\\"".to_string()
        } else {
            escapeChar(c)
        }
    }).collect::<Vec<_>>().join(""))
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

    __op_addadd(replicate(((3 - length(s))), '0').into_iter().collect(), s)
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
    let v = _0.chars().collect::<Vec<_>>();
    // ['\\', c, cs..]
    if v.len() > 2 && v[0] == '\\' {
        let c = v[1];
        let cs: String = v[2..].into_iter().collect();
        return match c {
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
                match head_q("bad escape sequence".to_string(),
                    (readHex(cs).read_s())) {
                    (i, cs_q) => (i, cs_q),
                }
            }
            _ => {
                match head_q("bad escape sequence".to_string(),
                                (readOct_q((__op_concat(c, cs))).read_s())) {
                    (i, cs_q) => (i, cs_q),
                }
            }
        }
    }

    // [c, cs..]
    if v.len() > 0 {
        let c = v[0];
        let cs: String = v[1..].into_iter().collect();
        return (c, cs)
    }

    // []
    __error!("unescape char: empty string".to_string())
}

pub fn readOct_q(s: String) -> Box<ReadS<char>> {

    let octStr = takeWhile_str(isOctDigit, take_str(3, s));

    let rest = drop_str((length(octStr)), s);

    box readOct(octStr).map(|(i, cs)| { (i, __op_addadd(cs, rest)) })
}

pub fn unescapeString(cs: String) -> String {
    if cs.len() == 0 {
        cs
    } else {
        match unescapeChar(cs) {
            (c, cs_q) => __op_concat(c, unescapeString(cs_q)),
        }
    }
}

pub fn sQuote(s: String) -> Box<ShowS> {
    box showString(format!("\'{}\'", s))
}

pub fn dQuote(s: String) -> Box<ShowS> {
    box showString(format!("\"{}\"", s))
}

pub fn head_q<a>(_0: String, _1: Vec<a>) -> a {
    if _1.is_empty() {
        __error!(_0);
    } else {
        _1.remove(0)
    }
}

pub fn head_q_str(_0: String, _1: String) -> char {
    if let Some(c) = _1.chars().next() {
       c 
    } else {
        __error!(_0);
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
            _phantom: PhantomData,
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
