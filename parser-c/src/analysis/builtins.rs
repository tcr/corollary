// Original file: "Builtins.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Analysis::DefTable;
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::TypeUtils;
// use Language::C::Data::Ident;
// use Language::C::Data::Node;

pub fn builtins() -> DefTable {

    let doTypeDef = |d| snd(defineTypeDef((identOfTypeDef(d)), d));

    let doIdent = |d| snd(defineGlobalIdent((declIdent(d)), d));

    let dName = |s| VarName((builtinIdent(s)), None);

    let param = |ty| {
        ParamDecl((VarDecl(NoName,
                           (DeclAttrs(noFunctionAttrs, (Auto(false)), vec![])),
                           ty)),
                  undefNode)
    };

    let fnAttrs = DeclAttrs(noFunctionAttrs, (FunLinkage(ExternalLinkage)), vec![]);

    let varAttrs = DeclAttrs(noFunctionAttrs, (Static(InternalLinkage, false)), vec![]);

    let fnType = |r, __as| FunctionType((FunType(r, (__map!(param, __as)), false)), noAttributes);

    let fnType_q = |r, __as| FunctionType((FunType(r, (__map!(param, __as)), true)), noAttributes);

    let func = |n, r, __as| {
        Declaration((Decl((VarDecl((dName(n)), fnAttrs, (fnType(r, __as)))), undefNode)))
    };

    let func_q = |n, r, __as| {
        Declaration((Decl((VarDecl((dName(n)), fnAttrs, (fnType_q(r, __as)))),
                          undefNode)))
    };

    let var = |n, t| Declaration((Decl((VarDecl((dName(n)), varAttrs, t)), undefNode)));

    let typedef = |n, t| TypeDef((builtinIdent(n)), t, vec![], undefNode);

    let typedefs = vec![typedef("__builtin_va_list".to_string(), valistType)];

    let idents = vec![func("__builtin_expect".to_string(),
                           (integral(TyLong)),
                           vec![integral(TyLong), integral(TyLong)]),
                      func("__builtin_bswap16".to_string(),
                           uint16_tType,
                           vec![uint16_tType]),
                      func("__builtin_bswap32".to_string(),
                           uint32_tType,
                           vec![uint32_tType]),
                      func("__builtin_bswap64".to_string(),
                           uint64_tType,
                           vec![uint64_tType]),
                      func("__builtin_fabs".to_string(),
                           (floating(TyDouble)),
                           vec![floating(TyDouble)]),
                      func("__builtin_fabsf".to_string(),
                           (floating(TyFloat)),
                           vec![floating(TyFloat)]),
                      func("__builtin_fabsl".to_string(),
                           (floating(TyLDouble)),
                           vec![floating(TyLDouble)]),
                      func("__builtin_inf".to_string(), (floating(TyDouble)), vec![]),
                      func("__builtin_inff".to_string(), (floating(TyFloat)), vec![]),
                      func("__builtin_infl".to_string(), (floating(TyLDouble)), vec![]),
                      func("__builtin_huge_val".to_string(),
                           (floating(TyDouble)),
                           vec![]),
                      func("__builtin_huge_valf".to_string(),
                           (floating(TyFloat)),
                           vec![]),
                      func("__builtin_huge_vall".to_string(),
                           (floating(TyLDouble)),
                           vec![]),
                      func("__builtin_copysign".to_string(),
                           (floating(TyDouble)),
                           vec![floating(TyDouble), floating(TyDouble)]),
                      func("__builtin_va_start".to_string(),
                           voidType,
                           vec![valistType, voidPtr]),
                      func("__builtin_va_end".to_string(), voidType, vec![valistType]),
                      func("__builtin_va_copy".to_string(),
                           voidType,
                           vec![valistType, valistType]),
                      func("__builtin_va_arg_pack".to_string(),
                           (integral(TyInt)),
                           vec![]),
                      func("__builtin_va_arg_pack_len".to_string(),
                           (integral(TyInt)),
                           vec![]),
                      func("__builtin_alloca".to_string(), voidPtr, vec![size_tType]),
                      func("__builtin_memcpy".to_string(),
                           voidPtr,
                           vec![voidPtr, constVoidPtr, size_tType]),
                      func("__builtin_strspn".to_string(),
                           size_tType,
                           vec![constCharPtr, constCharPtr]),
                      func("__builtin_strcspn".to_string(),
                           size_tType,
                           vec![constCharPtr, constCharPtr]),
                      func("__builtin_strchr".to_string(),
                           charPtr,
                           vec![constCharPtr, integral(TyInt)]),
                      func("__builtin_strncpy".to_string(),
                           charPtr,
                           vec![constCharPtr, constCharPtr, size_tType]),
                      func("__builtin_strncat".to_string(),
                           charPtr,
                           vec![constCharPtr, constCharPtr, size_tType]),
                      func("__builtin_strcmp".to_string(),
                           (integral(TyInt)),
                           vec![constCharPtr, constCharPtr]),
                      func("__builtin_strpbrk".to_string(),
                           charPtr,
                           vec![constCharPtr, constCharPtr]),
                      func("__builtin_bzero".to_string(),
                           voidType,
                           vec![voidPtr, size_tType]),
                      func("__builtin_clz".to_string(),
                           (integral(TyInt)),
                           vec![integral(TyUInt)]),
                      func("__builtin_constant_p".to_string(),
                           (integral(TyInt)),
                           vec![DirectType((TyBuiltin(TyAny)), noTypeQuals, noAttributes)]),
                      func("__builtin_extract_return_addr".to_string(),
                           voidPtr,
                           vec![voidPtr]),
                      func("__builtin_return_address".to_string(),
                           voidPtr,
                           vec![integral(TyUInt)]),
                      func("__builtin_frame_address".to_string(),
                           voidPtr,
                           vec![integral(TyUInt)]),
                      func("__builtin_expect".to_string(),
                           (integral(TyLong)),
                           vec![integral(TyLong), integral(TyLong)]),
                      func("__builtin_prefetch".to_string(),
                           voidType,
                           vec![constVoidPtr]),
                      var("__func__".to_string(), stringType),
                      var("__PRETTY_FUNCTION__".to_string(), stringType),
                      var("__FUNCTION__".to_string(), stringType),
                      func("__builtin_object_size".to_string(),
                           size_tType,
                           vec![voidPtr, integral(TyInt)]),
                      func("__builtin___memcpy_chk".to_string(),
                           voidPtr,
                           vec![voidPtr, constVoidPtr, size_tType, size_tType]),
                      func("__builtin___mempcpy_chk".to_string(),
                           voidPtr,
                           vec![voidPtr, constVoidPtr, size_tType, size_tType]),
                      func("__builtin___memmove_chk".to_string(),
                           voidPtr,
                           vec![voidPtr, constVoidPtr, size_tType, size_tType]),
                      func("__builtin___memset_chk".to_string(),
                           voidPtr,
                           vec![voidPtr, integral(TyInt), size_tType, size_tType]),
                      func("__builtin___strcpy_chk".to_string(),
                           charPtr,
                           vec![constCharPtr, constCharPtr, size_tType]),
                      func("__builtin___stpcpy_chk".to_string(),
                           charPtr,
                           vec![constCharPtr, constCharPtr, size_tType]),
                      func("__builtin___strncpy_chk".to_string(),
                           charPtr,
                           vec![constCharPtr, constCharPtr, size_tType, size_tType]),
                      func("__builtin___strcat_chk".to_string(),
                           charPtr,
                           vec![constCharPtr, constCharPtr, size_tType]),
                      func("__builtin___strncat_chk".to_string(),
                           charPtr,
                           vec![constCharPtr, constCharPtr, size_tType, size_tType]),
                      func_q("__builtin___sprintf_chk".to_string(),
                             (integral(TyInt)),
                             vec![charPtr, integral(TyInt), size_tType, constCharPtr]),
                      func_q("__builtin___snprintf_chk".to_string(),
                             (integral(TyInt)),
                             vec![charPtr,
                                  size_tType,
                                  integral(TyInt),
                                  size_tType,
                                  constCharPtr]),
                      func("__builtin___vsprintf_chk".to_string(),
                           (integral(TyInt)),
                           vec![charPtr,
                                integral(TyInt),
                                size_tType,
                                constCharPtr,
                                valistType]),
                      func("__builtin___vsnprintf_chk".to_string(),
                           (integral(TyInt)),
                           vec![charPtr,
                                size_tType,
                                integral(TyInt),
                                size_tType,
                                constCharPtr,
                                valistType])];

    foldr(doIdent, (foldr(doTypeDef, emptyDefTable, typedefs)), idents)
}
