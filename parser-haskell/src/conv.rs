use ast::*;

pub fn expr_to_pat(expr: &Expr) -> Pat {
    match *expr {
        Expr::Operator(ref s) => Pat::Operator(s.to_string()),
        Expr::Ref(ref id) => Pat::Ref(id.clone()),

        //TODO Parens is wrong? Don't make a span?
        Expr::Parens(ref inner) => {
            Pat::Span(inner.iter().map(|x| expr_to_pat(x)).collect::<Vec<_>>())
        },
        Expr::Span(ref inner) => {
            Pat::Span(inner.iter().map(|x| expr_to_pat(x)).collect::<Vec<_>>())
        }
        Expr::Vector(ref inner) => {
            Pat::Brackets(inner.iter().map(|x| expr_to_pat(x)).collect::<Vec<_>>())
        }

        Expr::Record(ref base, ref inner) => Pat::Dummy, // TODO
        Expr::Number(num) => Pat::Num(num),
        Expr::Str(ref s) => Pat::Str(s.clone()),
        Expr::Char(ref s) => Pat::Char(s.clone()),

        Expr::Dummy => Pat::Dummy,

        Expr::Op(..) |
        Expr::Case(..) |
        Expr::Let(..) |
        Expr::Do(..) |
        Expr::Lambda(..) |
        Expr::RecordArgs(..) |
        Expr::Error => {
            panic!("Invalid expr to pat conversion: {:?}", expr);
        }
    }
}
