use lalrpop_util::ParseError;
use lalrpop_util::ParseError::*;

/// Converts the default ParseError shape into something easier to handle.
pub fn simplify_parse_error<'input>(
    error: ParseError<usize, (usize, &'input str), ()>
    ) -> ParseError<usize, String, ()>
{
    match error {
        InvalidToken { location } => InvalidToken { location },
        UnrecognizedToken { token, expected } => {
            let token = token.map(|(start, (_, tok), end)| (start, tok.into(), end));
            UnrecognizedToken {token, expected}
        }
        ExtraToken { token: (start, (_, tok), end) } => {
            let token = (start, tok.into(), end);
            ExtraToken { token }
        },
        User { error } => User { error },
    }
}

fn code_error(code: &str, tok_pos: usize) {
    let code = format!("\n\n{}", code);
    let code = code.lines().collect::<Vec<_>>();
    let mut pos: isize = 0;
    for (i, lines) in (&code[..]).windows(3).enumerate() {
        if pos + lines[2].len() as isize >= tok_pos as isize {

            let arrow_len = (tok_pos as isize) - (pos - 6);
            let omit_left = if arrow_len > 60 { arrow_len as usize - 60 } else { 0 };

            // prints line no. and a 70-char window into line
            let print_line = |n: usize, mut line: &str| {
                if line.len() >= omit_left {
                    line = &line[omit_left..];
                    if line.len() > 70 {
                        line = &line[..70];
                    }
                }
                line = line.trim_right();
                if !line.is_empty() {
                    println!("{:>3} | {}", n, line);
                } else {
                    println!("{:>3} |", n);
                }
            };

            if i > 1 {
                print_line(i - 1, &lines[0]);
            }
            if i > 0 {
                print_line(i, &lines[1]);
            }
            print_line(i + 1, &lines[2]);

            if arrow_len > 0 {
                let arrow_len = arrow_len as usize - omit_left;
                println!("{}^", "~".repeat(arrow_len));
            }
            return;
        }
        pos += (lines[2].len() as isize) + 1;
    }
}

// Print out errors smartly
pub fn print_parse_error(code: &str, err: &ParseError<usize, String, ()>) {
    match *err {
        ParseError::InvalidToken { location: loc } => {
            println!("Error: Invalid token:");
            code_error(code, loc);
        }
        ParseError::UnrecognizedToken {
            token: Some((loc, ref tok, _)),
            ..
        } => {
            println!("Error: Unrecognized token `{}`:", tok);
            code_error(code, loc);
        }
        ParseError::ExtraToken { token: (loc, ref tok, _) } => {
            println!("Error: Extra token `{}`:", tok);
            code_error(code, loc);
        }
        _ => (),
    }
}
