use std::str::FromStr;
use ast::{Expr, Opcode, Ident, Statement};
use lalrpop_util::ErrorRecovery;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Exprs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Expr, Opcode, Ident, Statement};
    use lalrpop_util::ErrorRecovery;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3a_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22data_22(&'input str),
        Term_22deriving_22(&'input str),
        Term_22_7c_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt_28_3cExpr_3e_20_22_2c_22_29(Box<Expr>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<Box<Expr>>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<Box<Expr>>),
        Nt_28_3cIdent_3e_20_22_2c_22_29(Ident),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2a(::std::vec::Vec<Ident>),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2b(::std::vec::Vec<Ident>),
        Nt_28_3cIdent_3e_20_22_7c_22_29(Ident),
        Nt_28_3cIdent_3e_20_22_7c_22_29_2a(::std::vec::Vec<Ident>),
        Nt_28_3cIdent_3e_20_22_7c_22_29_2b(::std::vec::Vec<Ident>),
        NtComma_3cExpr_3e(Vec<Box<Expr>>),
        NtComma_3cIdent_3e(Vec<Ident>),
        NtData(Statement),
        NtData_2b(::std::vec::Vec<Statement>),
        NtExpr(Box<Expr>),
        NtExpr_3f(::std::option::Option<Box<Expr>>),
        NtExprOp(Opcode),
        NtExprs(Vec<Box<Expr>>),
        NtFactor(Box<Expr>),
        NtFactorOp(Opcode),
        NtIdent(Ident),
        NtIdent_3f(::std::option::Option<Ident>),
        NtNum(i32),
        NtPipe_3cIdent_3e(Vec<Ident>),
        NtStatements(Vec<Statement>),
        NtTerm(Box<Expr>),
        NtTier_3cExprOp_2c_20Factor_3e(Box<Expr>),
        NtTier_3cFactorOp_2c_20Term_3e(Box<Expr>),
        Nt____Exprs(Vec<Box<Expr>>),
        Nt____Statements(Vec<Statement>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13,
        // State 1
        11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, -50, -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 17, -28, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 20, -34, -34, -34, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30,
        // State 11
        0, 0, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, -4,
        // State 15
        11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13,
        // State 16
        -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, -31,
        // State 17
        -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, -32,
        // State 18
        11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13,
        // State 19
        -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, -35,
        // State 20
        -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, -36,
        // State 21
        0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -50, 0, -50, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -46, -46, -46, 0, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -52, -52, -52, 0, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -28, 0, 17, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -34, 20, -34, 0, -34, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30,
        // State 28
        0, -40, -40, -40, 0, -40, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -48, -48, -48, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, -5,
        // State 31
        0, 0, 0, -49, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30,
        // State 35
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30,
        // State 36
        0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, -49, 0, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -51, -51, -51, 0, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -47, -47, -47, 0, -47, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -17,
        -19,
        -33,
        -16,
        -53,
        -50,
        -46,
        -52,
        -28,
        -34,
        0,
        -40,
        -48,
        -18,
        -4,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -5,
        -49,
        -51,
        -47,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 5, 6, 0, 0, 0, 7, 0, 0, 8, 9, 10, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 8, 9, 10, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 25, 26, 27, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 7, 0, 0, 8, 0, 10, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 33, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 25, 26, 27, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 24, 0, 0, 25, 0, 27, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 39, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###""/""###,
            r###""::""###,
            r###""=""###,
            r###""data""###,
            r###""deriving""###,
            r###""|""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-zA-Z]+"#"###,
        ];
        __ACTION[(__state * 16)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Exprs<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Vec<Box<Expr>>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 16 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2d_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_3a_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22data_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22deriving_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_7c_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead.clone()),
                        expected: __expected_tokens(__state),
                    };
                    let mut __dropped_tokens = Vec::new();
                    loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __ACTION[(__state + 1) * 16 - 1];
                        if __action >= 0 {
                            break;
                        }
                        if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            return r;
                        }
                    }
                    let __original_state_len = __states.len();
                    let mut __error_state;
                    loop {
                        match __states.last().cloned() {
                            Some(__state) => {
                                __error_state = __ACTION[(__state as usize + 1) * 16 - 1];
                                if __error_state > 0  {
                                    break;
                                }
                                __states.pop();
                            }
                            None => {
                                return Err(__error);
                            }
                        }
                    }
                    let __start = __lookahead.0.clone();
                    let __end = __lookahead.2.clone();
                    loop {
                        if __ACTION[(__error_state as usize - 1) * 16 + __integer] != 0 {
                            let __new_len = __symbols.len() - (__original_state_len - __states.len());
                            __symbols.truncate(__new_len);
                            __states.push(__error_state - 1);
                            let __recovery = __lalrpop_util::ErrorRecovery {
                                error: __error,
                                dropped_tokens: __dropped_tokens,
                            };
                            __symbols.push((__start, __Symbol::Termerror(__recovery), __end));
                            continue '__inner;
                        }
                        __dropped_tokens.push(__lookahead);
                        __lookahead = match __tokens.next() {
                            Some(Ok(v)) => v,
                            None => break '__shift,
                            Some(Err(e)) => return Err(e),
                        };
                        __last_location = __lookahead.2.clone();
                        __integer = match __lookahead.1 {
                            (0, _) if true => 0,
                            (1, _) if true => 1,
                            (2, _) if true => 2,
                            (3, _) if true => 3,
                            (4, _) if true => 4,
                            (5, _) if true => 5,
                            (6, _) if true => 6,
                            (7, _) if true => 7,
                            (8, _) if true => 8,
                            (9, _) if true => 9,
                            (10, _) if true => 10,
                            (11, _) if true => 11,
                            (12, _) if true => 12,
                            (13, _) if true => 13,
                            (14, _) if true => 14,
                            _ => {
                                let __state = *__states.last().unwrap() as usize;
                                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                    token: Some(__lookahead),
                                    expected: __expected_tokens(__state),
                                };
                                return Err(__error);
                            }
                        };
                    }
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[(__state + 1) * 16 - 1];
                    if __action >= 0 {
                        break;
                    }
                    if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                }
                let __original_state_len = __states.len();
                let mut __error_state;
                loop {
                    match __states.last().cloned() {
                        Some(__state) => {
                            __error_state = __ACTION[(__state as usize + 1) * 16 - 1];
                            if __error_state > 0 && __EOF_ACTION[(__error_state as usize - 1)] != 0  {
                                break;
                            }
                            __states.pop();
                        }
                        None => {
                            return Err(__error);
                        }
                    }
                }
                let __new_len = __symbols.len() - (__original_state_len - __states.len());
                __symbols.truncate(__new_len);
                __states.push(__error_state - 1);
                let __recovery = __lalrpop_util::ErrorRecovery {
                    error: __error,
                    dropped_tokens: Vec::new(),
                };
                __symbols.push((__last_location.clone(), __Symbol::Termerror(__recovery), __last_location.clone()));
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Box<Expr>>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(30);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(29);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(45);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(46);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action46::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Ident> ",") = Ident, "," => ActionFn(38);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Ident> ",")* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Ident> ",")* = (<Ident> ",")+ => ActionFn(37);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Ident> ",")+ = Ident, "," => ActionFn(49);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action49::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Ident> ",")+ = (<Ident> ",")+, Ident, "," => ActionFn(50);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action50::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // (<Ident> "|") = Ident, "|" => ActionFn(33);
                let __sym1 = __pop_Term_22_7c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29(__nt), __end));
                6
            }
            12 => {
                // (<Ident> "|")* =  => ActionFn(31);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action31::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2a(__nt), __end));
                7
            }
            13 => {
                // (<Ident> "|")* = (<Ident> "|")+ => ActionFn(32);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2a(__nt), __end));
                7
            }
            14 => {
                // (<Ident> "|")+ = Ident, "|" => ActionFn(53);
                let __sym1 = __pop_Term_22_7c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action53::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__nt), __end));
                8
            }
            15 => {
                // (<Ident> "|")+ = (<Ident> "|")+, Ident, "|" => ActionFn(54);
                let __sym2 = __pop_Term_22_7c_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action54::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__nt), __end));
                8
            }
            16 => {
                // Comma<Expr> = Expr => ActionFn(57);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            17 => {
                // Comma<Expr> =  => ActionFn(58);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action58::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            18 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(59);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action59::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            19 => {
                // Comma<Expr> = (<Expr> ",")+ => ActionFn(60);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            20 => {
                // Comma<Ident> = Ident => ActionFn(61);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action61::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            21 => {
                // Comma<Ident> =  => ActionFn(62);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action62::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            22 => {
                // Comma<Ident> = (<Ident> ",")+, Ident => ActionFn(63);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action63::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            23 => {
                // Comma<Ident> = (<Ident> ",")+ => ActionFn(64);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action64::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            24 => {
                // Data = "data", Ident, "=", Pipe<Ident>, "deriving", "(", Comma<Ident>, ")" => ActionFn(4);
                let __sym7 = __pop_Term_22_29_22(__symbols);
                let __sym6 = __pop_NtComma_3cIdent_3e(__symbols);
                let __sym5 = __pop_Term_22_28_22(__symbols);
                let __sym4 = __pop_Term_22deriving_22(__symbols);
                let __sym3 = __pop_NtPipe_3cIdent_3e(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22data_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtData(__nt), __end));
                11
            }
            25 => {
                // Data = Ident, "::", Ident, "->", Ident => ActionFn(5);
                let __sym4 = __pop_NtIdent(__symbols);
                let __sym3 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym2 = __pop_NtIdent(__symbols);
                let __sym1 = __pop_Term_22_3a_3a_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtData(__nt), __end));
                11
            }
            26 => {
                // Data+ = Data => ActionFn(21);
                let __sym0 = __pop_NtData(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtData_2b(__nt), __end));
                12
            }
            27 => {
                // Data+ = Data+, Data => ActionFn(22);
                let __sym1 = __pop_NtData(__symbols);
                let __sym0 = __pop_NtData_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtData_2b(__nt), __end));
                12
            }
            28 => {
                // Expr = Tier<ExprOp, Factor> => ActionFn(7);
                let __sym0 = __pop_NtTier_3cExprOp_2c_20Factor_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                13
            }
            29 => {
                // Expr? = Expr => ActionFn(26);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                14
            }
            30 => {
                // Expr? =  => ActionFn(27);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action27::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                14
            }
            31 => {
                // ExprOp = "+" => ActionFn(9);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprOp(__nt), __end));
                15
            }
            32 => {
                // ExprOp = "-" => ActionFn(10);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprOp(__nt), __end));
                15
            }
            33 => {
                // Exprs = Comma<Expr> => ActionFn(2);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprs(__nt), __end));
                16
            }
            34 => {
                // Factor = Tier<FactorOp, Term> => ActionFn(8);
                let __sym0 = __pop_NtTier_3cFactorOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                17
            }
            35 => {
                // FactorOp = "*" => ActionFn(11);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactorOp(__nt), __end));
                18
            }
            36 => {
                // FactorOp = "/" => ActionFn(12);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactorOp(__nt), __end));
                18
            }
            37 => {
                // Ident = r#"[a-zA-Z]+"# => ActionFn(3);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                19
            }
            38 => {
                // Ident? = Ident => ActionFn(34);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                20
            }
            39 => {
                // Ident? =  => ActionFn(35);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action35::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                20
            }
            40 => {
                // Num = r#"[0-9]+"# => ActionFn(16);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                21
            }
            41 => {
                // Pipe<Ident> = Ident => ActionFn(65);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action65::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPipe_3cIdent_3e(__nt), __end));
                22
            }
            42 => {
                // Pipe<Ident> =  => ActionFn(66);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action66::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtPipe_3cIdent_3e(__nt), __end));
                22
            }
            43 => {
                // Pipe<Ident> = (<Ident> "|")+, Ident => ActionFn(67);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPipe_3cIdent_3e(__nt), __end));
                22
            }
            44 => {
                // Pipe<Ident> = (<Ident> "|")+ => ActionFn(68);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPipe_3cIdent_3e(__nt), __end));
                22
            }
            45 => {
                // Statements = Data+ => ActionFn(6);
                let __sym0 = __pop_NtData_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatements(__nt), __end));
                23
            }
            46 => {
                // Term = Num => ActionFn(13);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                24
            }
            47 => {
                // Term = "(", Expr, ")" => ActionFn(14);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                24
            }
            48 => {
                // Term = error => ActionFn(15);
                let __sym0 = __pop_Termerror(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                24
            }
            49 => {
                // Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => ActionFn(19);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_NtExprOp(__symbols);
                let __sym0 = __pop_NtTier_3cExprOp_2c_20Factor_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__nt), __end));
                25
            }
            50 => {
                // Tier<ExprOp, Factor> = Factor => ActionFn(20);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__nt), __end));
                25
            }
            51 => {
                // Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => ActionFn(17);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_NtFactorOp(__symbols);
                let __sym0 = __pop_NtTier_3cFactorOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__nt), __end));
                26
            }
            52 => {
                // Tier<FactorOp, Term> = Term => ActionFn(18);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__nt), __end));
                26
            }
            53 => {
                // __Exprs = Exprs => ActionFn(0);
                let __sym0 = __pop_NtExprs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            54 => {
                // __Statements = Statements => ActionFn(1);
                let __sym0 = __pop_NtStatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Statements(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22data_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22data_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22deriving_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22deriving_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_7c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cIdent_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cIdent_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtData<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtData(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtData_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtData_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactorOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactorOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPipe_3cIdent_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPipe_3cIdent_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStatements<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cExprOp_2c_20Factor_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cFactorOp_2c_20Term_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Exprs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Exprs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Statements<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Exprs::parse_Exprs;

mod __parse__Statements {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Expr, Opcode, Ident, Statement};
    use lalrpop_util::ErrorRecovery;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3a_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22data_22(&'input str),
        Term_22deriving_22(&'input str),
        Term_22_7c_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt_28_3cExpr_3e_20_22_2c_22_29(Box<Expr>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<Box<Expr>>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<Box<Expr>>),
        Nt_28_3cIdent_3e_20_22_2c_22_29(Ident),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2a(::std::vec::Vec<Ident>),
        Nt_28_3cIdent_3e_20_22_2c_22_29_2b(::std::vec::Vec<Ident>),
        Nt_28_3cIdent_3e_20_22_7c_22_29(Ident),
        Nt_28_3cIdent_3e_20_22_7c_22_29_2a(::std::vec::Vec<Ident>),
        Nt_28_3cIdent_3e_20_22_7c_22_29_2b(::std::vec::Vec<Ident>),
        NtComma_3cExpr_3e(Vec<Box<Expr>>),
        NtComma_3cIdent_3e(Vec<Ident>),
        NtData(Statement),
        NtData_2b(::std::vec::Vec<Statement>),
        NtExpr(Box<Expr>),
        NtExpr_3f(::std::option::Option<Box<Expr>>),
        NtExprOp(Opcode),
        NtExprs(Vec<Box<Expr>>),
        NtFactor(Box<Expr>),
        NtFactorOp(Opcode),
        NtIdent(Ident),
        NtIdent_3f(::std::option::Option<Ident>),
        NtNum(i32),
        NtPipe_3cIdent_3e(Vec<Ident>),
        NtStatements(Vec<Statement>),
        NtTerm(Box<Expr>),
        NtTier_3cExprOp_2c_20Factor_3e(Box<Expr>),
        NtTier_3cFactorOp_2c_20Term_3e(Box<Expr>),
        Nt____Exprs(Vec<Box<Expr>>),
        Nt____Statements(Vec<Statement>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 7, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 7, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 19, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 19, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 23, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, -37, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 25, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, -14, 0,
        // State 23
        26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, -15, 0,
        // State 25
        0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 26
        0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 27
        0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -20, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -37, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -22, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, 0,
        // State 32
        0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0,
        // State 33
        0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -26,
        -45,
        0,
        -54,
        0,
        0,
        -27,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -25,
        -37,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -24,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 18, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###""/""###,
            r###""::""###,
            r###""=""###,
            r###""data""###,
            r###""deriving""###,
            r###""|""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-zA-Z]+"#"###,
        ];
        __ACTION[(__state * 16)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Statements<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Vec<Statement>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 16 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2d_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_3a_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22data_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22deriving_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_7c_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Statement>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(30);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(29);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(45);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(46);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action46::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Ident> ",") = Ident, "," => ActionFn(38);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Ident> ",")* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Ident> ",")* = (<Ident> ",")+ => ActionFn(37);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Ident> ",")+ = Ident, "," => ActionFn(49);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action49::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Ident> ",")+ = (<Ident> ",")+, Ident, "," => ActionFn(50);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action50::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // (<Ident> "|") = Ident, "|" => ActionFn(33);
                let __sym1 = __pop_Term_22_7c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29(__nt), __end));
                6
            }
            12 => {
                // (<Ident> "|")* =  => ActionFn(31);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action31::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2a(__nt), __end));
                7
            }
            13 => {
                // (<Ident> "|")* = (<Ident> "|")+ => ActionFn(32);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2a(__nt), __end));
                7
            }
            14 => {
                // (<Ident> "|")+ = Ident, "|" => ActionFn(53);
                let __sym1 = __pop_Term_22_7c_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action53::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__nt), __end));
                8
            }
            15 => {
                // (<Ident> "|")+ = (<Ident> "|")+, Ident, "|" => ActionFn(54);
                let __sym2 = __pop_Term_22_7c_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action54::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__nt), __end));
                8
            }
            16 => {
                // Comma<Expr> = Expr => ActionFn(57);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            17 => {
                // Comma<Expr> =  => ActionFn(58);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action58::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            18 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(59);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action59::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            19 => {
                // Comma<Expr> = (<Expr> ",")+ => ActionFn(60);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action60::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                9
            }
            20 => {
                // Comma<Ident> = Ident => ActionFn(61);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action61::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            21 => {
                // Comma<Ident> =  => ActionFn(62);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action62::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            22 => {
                // Comma<Ident> = (<Ident> ",")+, Ident => ActionFn(63);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action63::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            23 => {
                // Comma<Ident> = (<Ident> ",")+ => ActionFn(64);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action64::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cIdent_3e(__nt), __end));
                10
            }
            24 => {
                // Data = "data", Ident, "=", Pipe<Ident>, "deriving", "(", Comma<Ident>, ")" => ActionFn(4);
                let __sym7 = __pop_Term_22_29_22(__symbols);
                let __sym6 = __pop_NtComma_3cIdent_3e(__symbols);
                let __sym5 = __pop_Term_22_28_22(__symbols);
                let __sym4 = __pop_Term_22deriving_22(__symbols);
                let __sym3 = __pop_NtPipe_3cIdent_3e(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22data_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtData(__nt), __end));
                11
            }
            25 => {
                // Data = Ident, "::", Ident, "->", Ident => ActionFn(5);
                let __sym4 = __pop_NtIdent(__symbols);
                let __sym3 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym2 = __pop_NtIdent(__symbols);
                let __sym1 = __pop_Term_22_3a_3a_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtData(__nt), __end));
                11
            }
            26 => {
                // Data+ = Data => ActionFn(21);
                let __sym0 = __pop_NtData(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtData_2b(__nt), __end));
                12
            }
            27 => {
                // Data+ = Data+, Data => ActionFn(22);
                let __sym1 = __pop_NtData(__symbols);
                let __sym0 = __pop_NtData_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtData_2b(__nt), __end));
                12
            }
            28 => {
                // Expr = Tier<ExprOp, Factor> => ActionFn(7);
                let __sym0 = __pop_NtTier_3cExprOp_2c_20Factor_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                13
            }
            29 => {
                // Expr? = Expr => ActionFn(26);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                14
            }
            30 => {
                // Expr? =  => ActionFn(27);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action27::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtExpr_3f(__nt), __end));
                14
            }
            31 => {
                // ExprOp = "+" => ActionFn(9);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprOp(__nt), __end));
                15
            }
            32 => {
                // ExprOp = "-" => ActionFn(10);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprOp(__nt), __end));
                15
            }
            33 => {
                // Exprs = Comma<Expr> => ActionFn(2);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprs(__nt), __end));
                16
            }
            34 => {
                // Factor = Tier<FactorOp, Term> => ActionFn(8);
                let __sym0 = __pop_NtTier_3cFactorOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                17
            }
            35 => {
                // FactorOp = "*" => ActionFn(11);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactorOp(__nt), __end));
                18
            }
            36 => {
                // FactorOp = "/" => ActionFn(12);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactorOp(__nt), __end));
                18
            }
            37 => {
                // Ident = r#"[a-zA-Z]+"# => ActionFn(3);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                19
            }
            38 => {
                // Ident? = Ident => ActionFn(34);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                20
            }
            39 => {
                // Ident? =  => ActionFn(35);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action35::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtIdent_3f(__nt), __end));
                20
            }
            40 => {
                // Num = r#"[0-9]+"# => ActionFn(16);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                21
            }
            41 => {
                // Pipe<Ident> = Ident => ActionFn(65);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action65::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPipe_3cIdent_3e(__nt), __end));
                22
            }
            42 => {
                // Pipe<Ident> =  => ActionFn(66);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action66::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtPipe_3cIdent_3e(__nt), __end));
                22
            }
            43 => {
                // Pipe<Ident> = (<Ident> "|")+, Ident => ActionFn(67);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action67::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPipe_3cIdent_3e(__nt), __end));
                22
            }
            44 => {
                // Pipe<Ident> = (<Ident> "|")+ => ActionFn(68);
                let __sym0 = __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPipe_3cIdent_3e(__nt), __end));
                22
            }
            45 => {
                // Statements = Data+ => ActionFn(6);
                let __sym0 = __pop_NtData_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatements(__nt), __end));
                23
            }
            46 => {
                // Term = Num => ActionFn(13);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                24
            }
            47 => {
                // Term = "(", Expr, ")" => ActionFn(14);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                24
            }
            48 => {
                // Term = error => ActionFn(15);
                let __sym0 = __pop_Termerror(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                24
            }
            49 => {
                // Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => ActionFn(19);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_NtExprOp(__symbols);
                let __sym0 = __pop_NtTier_3cExprOp_2c_20Factor_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__nt), __end));
                25
            }
            50 => {
                // Tier<ExprOp, Factor> = Factor => ActionFn(20);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__nt), __end));
                25
            }
            51 => {
                // Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => ActionFn(17);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_NtFactorOp(__symbols);
                let __sym0 = __pop_NtTier_3cFactorOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__nt), __end));
                26
            }
            52 => {
                // Tier<FactorOp, Term> = Term => ActionFn(18);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__nt), __end));
                26
            }
            53 => {
                // __Exprs = Exprs => ActionFn(0);
                let __sym0 = __pop_NtExprs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Exprs(__nt), __end));
                27
            }
            54 => {
                // __Statements = Statements => ActionFn(1);
                let __sym0 = __pop_NtStatements(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22data_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22data_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22deriving_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22deriving_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_7c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cIdent_3e_20_22_7c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cIdent_3e_20_22_7c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cIdent_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cIdent_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtData<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtData(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtData_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtData_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactorOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactorOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPipe_3cIdent_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ident>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPipe_3cIdent_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStatements<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStatements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cExprOp_2c_20Factor_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cFactorOp_2c_20Term_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Exprs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Exprs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Statements<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Statement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Statements(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Statements::parse_Statements;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_state = 9;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        124 => /* '|' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        62 => /* '>' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        98 ... 100 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((13, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 117 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        118 => /* 'v' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        119 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 102 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        103 => /* 'g' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        104 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Statement>, usize),
) -> Vec<Statement>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Ident
{
    Ident((__0).to_string())
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, Ident, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Vec<Ident>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, d, _): (usize, Vec<Ident>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    Statement::Data(id, p, d)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Ident, usize),
    (_, __3, _): (usize, &'input str, usize),
    (_, __4, _): (usize, Ident, usize),
) -> Statement
{
    Statement::Dummy
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
) -> Vec<Statement>
{
    v
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Add
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Sub
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Mul
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Div
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> Box<Expr>
{
    Box::new(Expr::Number(__0))
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize),
) -> Box<Expr>
{
    { errors.push(__0); Box::new(Expr::Error) }
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Opcode, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Opcode, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> ::std::vec::Vec<Statement>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
    (_, e, _): (usize, Statement, usize),
) -> ::std::vec::Vec<Statement>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
    (_, e, _): (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    match e { // (1)
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
    (_, e, _): (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    match e { // (1)
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    match e { // (1)
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::option::Option<Box<Expr>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Box<Expr>>
{
    None
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    v
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Ident>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
) -> ::std::vec::Vec<Ident>
{
    v
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Ident
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
) -> ::std::option::Option<Ident>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Ident>
{
    None
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Ident>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
) -> ::std::vec::Vec<Ident>
{
    v
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Ident
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
) -> ::std::vec::Vec<Ident>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
    (_, e, _): (usize, Ident, usize),
) -> ::std::vec::Vec<Ident>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
) -> ::std::vec::Vec<Ident>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ident>, usize),
    (_, e, _): (usize, Ident, usize),
) -> ::std::vec::Vec<Ident>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action30(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action30(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action28(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        errors,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, ::std::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        errors,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action49<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, Ident, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action50<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, Ident, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Ident>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action38(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action51<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action36(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        errors,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action52<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        errors,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action53<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, Ident, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action33(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action54<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, Ident, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Ident>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action33(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action55<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action31(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        errors,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action56<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, ::std::option::Option<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action32(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        errors,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action57<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action58<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Box<Expr>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action27(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action59<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action26(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action60<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action61<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, Ident, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action62<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Ident>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action35(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action63<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, Ident, usize),
) -> Vec<Ident>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action64<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action65<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, Ident, usize),
) -> Vec<Ident>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action66<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Ident>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action35(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action67<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
    __1: (usize, Ident, usize),
) -> Vec<Ident>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action68<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Ident>, usize),
) -> Vec<Ident>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        errors,
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, 'err, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
