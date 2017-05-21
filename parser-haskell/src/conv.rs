use ast::*;

pub fn expr_to_pat(expr: &Expr) -> Pat {
    match *expr {
        //Expr::Op(Box<Expr>, String, Box<Expr>),
        //Expr::Operator(String),
        Expr::Ref(ref id) => Pat::Ref(id.clone()),

        //TODO Parens is wrong:
        Expr::Parens(ref inner) => Pat::Span(inner.iter().map(|x| expr_to_pat(x)).collect::<Vec<_>>()),
        Expr::Span(ref inner) => Pat::Span(inner.iter().map(|x| expr_to_pat(x)).collect::<Vec<_>>()),
        //Expr::Vector(Vec<Expr>),

        //Expr::Record(Vec<(Ident, Expr)>),
        Expr::Number(num) => Pat::Num(num),
        Expr::Str(ref s) => Pat::Str(s.clone()),
        Expr::Char(ref s) => Pat::Char(s.clone()),

        //Expr::Case(Box<Expr>, Vec<CaseCond>),
        //Expr::Let(Vec<Statement>, Vec<Statement>),
        //Expr::Do(Vec<Statement>, Vec<Statement>),
        Expr::Lambda |
        Expr::Error |
        Expr::Dummy |
        _ => {
            panic!("Invalid expr to pat conversion: {:?}", expr);
        }
    }
}
