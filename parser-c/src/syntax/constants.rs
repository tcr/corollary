#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Bits;
// use Data::Char;
// use Numeric;
// use showOct;
// use Language::C::Data::Node;
// use Language::C::Data::Position;
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
pub struct CString(Vec<Char>, bool);


#[derive(Clone, Debug, Eq, Ord)]
pub struct Flags<f>(Integer);


pub fn _showWideFlag<a>(flag: bool) -> ShowS {
    if flag {     
showString("L".to_string())} else {
id
    }
}

pub fn cChar<a>(c: Char) -> CChar {
    CChar(c, false)
}

pub fn cChar_w<a>(c: Char) -> CChar {
    CChar(c, true)
}

pub fn cChars<a>() -> CChar {
    CChars
}

pub fn cFloat<a>() -> CFloat {
    CFloat(show)
}

pub fn cInteger<a>(i: Integer) -> CInteger {
    CInteger(i, DecRepr, noFlags)
}

pub fn cString<a>(__str: String) -> CString {
    CString(__str, false)
}

pub fn cString_w<a>(__str: String) -> CString {
    CString(__str, true)
}

pub fn clearFlag<a>(flag: f, Flags(k): Flags<f>) -> Flags<f> {
    Flags(clearBit(k, fromEnum(flag)))
}

pub fn concatCStrings<a>(cs: Vec<CString>) -> CString {
    CString((concatMap(getCString, cs)), (any(isWideString, cs)))
}

pub fn dQuote<a>(s: String, t: ShowS) -> ShowS {
    __op_addadd((__op_concat('\"', s)), __op_addadd("\"".to_string(), t))
}

pub fn escapeCChar<a>(_0: Char) -> String {
    match (_0) {
        '\'' => {
            "\\\'".to_string()
        },
        c => {
            /* Expr::Error */ Error
        },
    }
}

pub fn escapeChar<a>(_0: Char) -> String {
    match (_0) {
        '\\' => {
            "\\\\".to_string()
        },
        '\u{7}' => {
            "\\a".to_string()
        },
        '\u{8}' => {
            "\\b".to_string()
        },
        '\u{1b}' => {
            "\\e".to_string()
        },
        '\u{c}' => {
            "\\f".to_string()
        },
        '\n' => {
            "\\n".to_string()
        },
        '\r' => {
            "\\r".to_string()
        },
        '\t' => {
            "\\t".to_string()
        },
        '\u{b}' => {
            "\\v".to_string()
        },
        c => {
            /* Expr::Error */ Error
        },
    }
}

pub fn getCChar<a>(_0: CChar) -> Vec<Char> {
    match (_0) {
        CChar(c, _) => {
            vec![c]
        },
        CChars(cs, _) => {
            cs
        },
    }
}

pub fn getCCharAsInt<a>(_0: CChar) -> Integer {
    match (_0) {
        CChar(c, _) => {
            fromIntegral((fromEnum(c)))
        },
        CChars(_cs, _) => {
            __error!("integer value of multi-character character constants is implementation defined".to_string())
        },
    }
}

pub fn getCInteger<a>(CInteger(i, _, _): CInteger) -> Integer {
    i
}

pub fn getCString<a>(CString(__str, _): CString) -> String {
    __str
}

pub fn head_q<a>(_0: String, _1: Vec<a>) -> a {
    match (_0, _1) {
        (err, []) => {
            __error!(err)
        },
        (_, [x, _]) => {
            x
        },
    }
}

pub fn isAsciiSourceChar<a>(c: Char) -> bool {
    (isAscii(c) && isPrint(c))
}

pub fn isCChar<a>(_0: Char) -> bool {
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
            isAsciiSourceChar(c)
        },
    }
}

pub fn isSChar<a>(_0: Char) -> bool {
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
            isAsciiSourceChar(c)
        },
    }
}

pub fn isWideChar<a>(_0: CChar) -> bool {
    match (_0) {
        CChar(_, wideFlag) => {
            wideFlag
        },
        CChars(_, wideFlag) => {
            wideFlag
        },
    }
}

pub fn isWideString<a>(CString(_, wideflag): CString) -> bool {
    wideflag
}

pub fn noFlags<a>() -> Flags<f> {
    Flags(0)
}

pub fn readCFloat<a>() -> CFloat {
    CFloat
}

pub fn readCInteger<a>(repr: CIntRepr, __str: String) -> Either<String, CInteger> {
    match readNum(__str) {
        [(n, suffix)] => {
            mkCInt(n, suffix)
        },
        parseFailed => {
            Left(__op_addadd("Bad Integer literal: ".to_string(), show(parseFailed)))
        },
    }
}

pub fn sQuote<a>(s: String, t: ShowS) -> ShowS {
    __op_addadd("\'".to_string(), __op_addadd(s, __op_addadd("\'".to_string(), t)))
}

pub fn setFlag<a>(flag: f, Flags(k): Flags<f>) -> Flags<f> {
    Flags(setBit(k, fromEnum(flag)))
}

pub fn showCharConst<a>(c: Char) -> ShowS {
    sQuote(escapeCChar(c))
}

pub fn showStringLit<a>() -> ShowS {
    dQuote(concatMap(showStringChar))
}

pub fn testFlag<a>(flag: f, Flags(k): Flags<f>) -> bool {
    testBit(k, fromEnum(flag))
}

pub fn unescapeChar<a>(_0: String) -> (Char, String) {
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
                    match head_q("bad escape sequence".to_string(), (readOct((__op_concat(c, cs))))) {
                        (i, cs_q) => {
                            (toEnum(i), cs_q)
                        },
                    }
                },
            }
        },
        [c, cs] => {
            (c, cs)
        },
        [] => {
            __error!("unescape char: empty string".to_string())
        },
    }
}

pub fn unescapeString<a>(_0: String) -> String {
    match (_0) {
        [] => {
            vec![]
        },
        cs => {
            match unescapeChar(cs) {
                (c, cs_q) => {
                    __op_concat(c, unescapeString(cs_q))
                },
            }
        },
    }
}



