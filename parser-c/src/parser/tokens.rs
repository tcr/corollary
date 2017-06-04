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
    CTokAlignas(PosLength),
    CTokAsm(PosLength),
    CTokAtomic(PosLength),
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
    CTokGeneric(PosLength),
    CTokGoto(PosLength),
    CTokIf(PosLength),
    CTokInline(PosLength),
    CTokInt(PosLength),
    CTokInt128(PosLength),
    CTokLong(PosLength),
    CTokLabel(PosLength),
    CTokNoreturn(PosLength),
    CTokNullable(PosLength),
    CTokNonnull(PosLength),
    CTokRegister(PosLength),
    CTokRestrict(PosLength),
    CTokReturn(PosLength),
    CTokShort(PosLength),
    CTokSigned(PosLength),
    CTokSizeof(PosLength),
    CTokStatic(PosLength),
    CTokStaticAssert(PosLength),
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
    CTokClangC(PosLength, ClangCTok),
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

pub struct ClangCTok(ClangCVersion);


pub fn posLenOfTok(_0: CToken) -> (Position, isize) {
    match (_0) {
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
        _0 => {
            pos
        },
    }
}



