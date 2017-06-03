// Original file: "Tokens.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::Position;
// use Position;
// use Language::C::Data::Ident;
// use Ident;
// use Language::C::Syntax::Constants;
// use CChar;

pub enum CToken {
    CTokLParen(PosLength),
    CTokRParen(PosLength),
    CTokLBracket(PosLength),
    CTokRBracket(PosLength),
    CTokArrow(PosLength),
    CTokDot(PosLength),
    CTokExclam(PosLength),
    CTokTilde(PosLength),
    CTokInc(PosLength),
    CTokDec(PosLength),
    CTokPlus(PosLength),
    CTokMinus(PosLength),
    CTokStar(PosLength),
    CTokSlash(PosLength),
    CTokPercent(PosLength),
    CTokAmper(PosLength),
    CTokShiftL(PosLength),
    CTokShiftR(PosLength),
    CTokLess(PosLength),
    CTokLessEq(PosLength),
    CTokHigh(PosLength),
    CTokHighEq(PosLength),
    CTokEqual(PosLength),
    CTokUnequal(PosLength),
    CTokHat(PosLength),
    CTokBar(PosLength),
    CTokAnd(PosLength),
    CTokOr(PosLength),
    CTokQuest(PosLength),
    CTokColon(PosLength),
    CTokAssign(PosLength),
    CTokPlusAss(PosLength),
    CTokMinusAss(PosLength),
    CTokStarAss(PosLength),
    CTokSlashAss(PosLength),
    CTokPercAss(PosLength),
    CTokAmpAss(PosLength),
    CTokHatAss(PosLength),
    CTokBarAss(PosLength),
    CTokSLAss(PosLength),
    CTokSRAss(PosLength),
    CTokComma(PosLength),
    CTokSemic(PosLength),
    CTokLBrace(PosLength),
    CTokRBrace(PosLength),
    CTokEllipsis(PosLength),
    CTokAlignof(PosLength),
    CTokAsm(PosLength),
    CTokAuto(PosLength),
    CTokBreak(PosLength),
    CTokBool(PosLength),
    CTokCase(PosLength),
    CTokChar(PosLength),
    CTokConst(PosLength),
    CTokContinue(PosLength),
    CTokComplex(PosLength),
    CTokDefault(PosLength),
    CTokDo(PosLength),
    CTokDouble(PosLength),
    CTokElse(PosLength),
    CTokEnum(PosLength),
    CTokExtern(PosLength),
    CTokFloat(PosLength),
    CTokFor(PosLength),
    CTokGoto(PosLength),
    CTokIf(PosLength),
    CTokInline(PosLength),
    CTokInt(PosLength),
    CTokLong(PosLength),
    CTokLabel(PosLength),
    CTokRegister(PosLength),
    CTokRestrict(PosLength),
    CTokReturn(PosLength),
    CTokShort(PosLength),
    CTokSigned(PosLength),
    CTokSizeof(PosLength),
    CTokStatic(PosLength),
    CTokStruct(PosLength),
    CTokSwitch(PosLength),
    CTokTypedef(PosLength),
    CTokTypeof(PosLength),
    CTokThread(PosLength),
    CTokUnion(PosLength),
    CTokUnsigned(PosLength),
    CTokVoid(PosLength),
    CTokVolatile(PosLength),
    CTokWhile(PosLength),
    CTokCLit(PosLength, CChar),
    CTokILit(PosLength, CInteger),
    CTokFLit(PosLength, CFloat),
    CTokSLit(PosLength, CString),
    CTokIdent(PosLength, Ident),
    CTokTyIdent(PosLength, Ident),
    CTokGnuC(GnuCTok, PosLength),
    CTokEof
}
pub use self::CToken::*;

pub enum GnuCTok {
    GnuCAttrTok,
    GnuCExtTok,
    GnuCVaArg,
    GnuCOffsetof,
    GnuCTyCompat,
    GnuCComplexReal,
    GnuCComplexImag
}
pub use self::GnuCTok::*;

pub fn posLenOfTok(_0: CToken) -> (Position, isize) {
    match (_0) {
        CTokLParen(pos) => {
            pos
        },
        CTokRParen(pos) => {
            pos
        },
        CTokLBracket(pos) => {
            pos
        },
        CTokRBracket(pos) => {
            pos
        },
        CTokArrow(pos) => {
            pos
        },
        CTokDot(pos) => {
            pos
        },
        CTokExclam(pos) => {
            pos
        },
        CTokTilde(pos) => {
            pos
        },
        CTokInc(pos) => {
            pos
        },
        CTokDec(pos) => {
            pos
        },
        CTokPlus(pos) => {
            pos
        },
        CTokMinus(pos) => {
            pos
        },
        CTokStar(pos) => {
            pos
        },
        CTokSlash(pos) => {
            pos
        },
        CTokPercent(pos) => {
            pos
        },
        CTokAmper(pos) => {
            pos
        },
        CTokShiftL(pos) => {
            pos
        },
        CTokShiftR(pos) => {
            pos
        },
        CTokLess(pos) => {
            pos
        },
        CTokLessEq(pos) => {
            pos
        },
        CTokHigh(pos) => {
            pos
        },
        CTokHighEq(pos) => {
            pos
        },
        CTokEqual(pos) => {
            pos
        },
        CTokUnequal(pos) => {
            pos
        },
        CTokHat(pos) => {
            pos
        },
        CTokBar(pos) => {
            pos
        },
        CTokAnd(pos) => {
            pos
        },
        CTokOr(pos) => {
            pos
        },
        CTokQuest(pos) => {
            pos
        },
        CTokColon(pos) => {
            pos
        },
        CTokAssign(pos) => {
            pos
        },
        CTokPlusAss(pos) => {
            pos
        },
        CTokMinusAss(pos) => {
            pos
        },
        CTokStarAss(pos) => {
            pos
        },
        CTokSlashAss(pos) => {
            pos
        },
        CTokPercAss(pos) => {
            pos
        },
        CTokAmpAss(pos) => {
            pos
        },
        CTokHatAss(pos) => {
            pos
        },
        CTokBarAss(pos) => {
            pos
        },
        CTokSLAss(pos) => {
            pos
        },
        CTokSRAss(pos) => {
            pos
        },
        CTokComma(pos) => {
            pos
        },
        CTokSemic(pos) => {
            pos
        },
        CTokLBrace(pos) => {
            pos
        },
        CTokRBrace(pos) => {
            pos
        },
        CTokEllipsis(pos) => {
            pos
        },
        CTokAlignof(pos) => {
            pos
        },
        CTokAsm(pos) => {
            pos
        },
        CTokAuto(pos) => {
            pos
        },
        CTokBreak(pos) => {
            pos
        },
        CTokBool(pos) => {
            pos
        },
        CTokCase(pos) => {
            pos
        },
        CTokChar(pos) => {
            pos
        },
        CTokConst(pos) => {
            pos
        },
        CTokContinue(pos) => {
            pos
        },
        CTokComplex(pos) => {
            pos
        },
        CTokDefault(pos) => {
            pos
        },
        CTokDo(pos) => {
            pos
        },
        CTokDouble(pos) => {
            pos
        },
        CTokElse(pos) => {
            pos
        },
        CTokEnum(pos) => {
            pos
        },
        CTokExtern(pos) => {
            pos
        },
        CTokFloat(pos) => {
            pos
        },
        CTokFor(pos) => {
            pos
        },
        CTokGoto(pos) => {
            pos
        },
        CTokInt(pos) => {
            pos
        },
        CTokInline(pos) => {
            pos
        },
        CTokIf(pos) => {
            pos
        },
        CTokLong(pos) => {
            pos
        },
        CTokLabel(pos) => {
            pos
        },
        CTokRegister(pos) => {
            pos
        },
        CTokRestrict(pos) => {
            pos
        },
        CTokReturn(pos) => {
            pos
        },
        CTokShort(pos) => {
            pos
        },
        CTokSigned(pos) => {
            pos
        },
        CTokSizeof(pos) => {
            pos
        },
        CTokStatic(pos) => {
            pos
        },
        CTokStruct(pos) => {
            pos
        },
        CTokSwitch(pos) => {
            pos
        },
        CTokTypedef(pos) => {
            pos
        },
        CTokTypeof(pos) => {
            pos
        },
        CTokThread(pos) => {
            pos
        },
        CTokUnion(pos) => {
            pos
        },
        CTokUnsigned(pos) => {
            pos
        },
        CTokVoid(pos) => {
            pos
        },
        CTokVolatile(pos) => {
            pos
        },
        CTokWhile(pos) => {
            pos
        },
        CTokCLit(pos, _) => {
            pos
        },
        CTokILit(pos, _) => {
            pos
        },
        CTokFLit(pos, _) => {
            pos
        },
        CTokSLit(pos, _) => {
            pos
        },
        CTokIdent(pos, _) => {
            pos
        },
        CTokTyIdent(pos, _) => {
            pos
        },
        CTokGnuC(_, pos) => {
            pos
        },
        CTokEof => {
            __error!("tokenPos: Eof".to_string())
        },
    }
}



