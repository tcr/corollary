use haskell_support::*;

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
struct CInteger(CInteger<Integer, CIntRepr, Flags<CIntFlag>>);

#[derive(Clone, Debug, Eq, Ord)]
struct CFloat(CFloat<String>);

#[derive(Clone, Debug, Eq, Ord)]
struct CString(CString<Vec<Char>, bool>);

pub fn _showWideFlag(flag: bool) -> ShowS {
    __TODO_if(flag, then, showString, "L".to_string(), __TODO_else, id)
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

pub fn escapeCChar(__0: Char) -> String {
    match (__0) {
        '\'' => {
            "\\\'".to_string()
        },
        c => {
            <Expr::Dummy>
        },
    }
}

pub fn escapeChar(__0: Char) -> String {
    match (__0) {
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
            <Expr::Dummy>
        },
    }
}

pub fn getCChar(__0: CChar) -> Vec<Char> {
    match (__0) {
        CChar(c, _) => {
            vec![c]
        },
        CChars(cs, _) => {
            cs
        },
    }
}

pub fn getCCharAsInt(__0: CChar) -> Integer {
    match (__0) {
        CChar(c, _) => {
            fromIntegral((fromEnum(c)))
        },
        CChars(_cs, _) => {
            __error!("integer value of multi-character character constants is implementation defined".to_string())
        },
    }
}

pub fn getCInteger(CInteger(i, _, _): CInteger) -> Integer {
    i
}

pub fn getCString(CString(__str, _): CString) -> String {
    __str
}

pub fn head_q(__0: String, __1: Vec<a>) -> a {
    match (__0, __1) {
        (err, []) => {
            __error!(err)
        },
        (_, [x, ..._]) => {
            x
        },
    }
}

pub fn isAsciiSourceChar(c: Char) -> bool {
    (isAscii(c) && isPrint(c))
}

pub fn isCChar(__0: Char) -> bool {
    match (__0) {
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

pub fn isSChar(__0: Char) -> bool {
    match (__0) {
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

pub fn isWideChar(__0: CChar) -> bool {
    match (__0) {
        CChar(_, wideFlag) => {
            wideFlag
        },
        CChars(_, wideFlag) => {
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

pub fn sQuote(s: String, t: ShowS) -> ShowS {
    __op_addadd("\'".to_string(), __op_addadd(s, __op_addadd("\'".to_string(), t)))
}

pub fn setFlag(flag: f, Flags(k): Flags<f>) -> Flags<f> {
    Flags(setBit(k, fromEnum(flag)))
}

pub fn showCharConst(c: Char) -> ShowS {
    sQuote(escapeCChar(c))
}

pub fn showStringLit() -> ShowS {
    dQuote(concatMap(showStringChar))
}

pub fn testFlag(flag: f, Flags(k): Flags<f>) -> bool {
    testBit(k, fromEnum(flag))
}

pub fn unescapeChar(__0: String) -> (Char, String) {
    match (__0) {
        ['\\', ...[c, ...cs]] => {
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
        [c, ...cs] => {
            (c, cs)
        },
        [] => {
            __error!("unescape char: empty string".to_string())
        },
    }
}

pub fn unescapeString(__0: String) -> String {
    match (__0) {
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

