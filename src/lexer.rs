use tokens::{Function, Number, Operator, Token, TokenList, Variable};

fn parse_implicit(expr: &str) -> TokenList {
    let mut tokens: TokenList = Vec::with_capacity(expr.len());
    let mut temp = String::new();
    let mut chars_left = expr.len();

    for ch in expr.chars() {
        if ch.is_digit(10) {
            temp.push(ch);
            if chars_left > 1 {
                chars_left -= 1;
                continue;
            }
        }

        if !temp.is_empty() {
            tokens.push(Token::Number(Number::new(
                temp.parse::<f64>().unwrap(),
            )));

            temp.clear();
        }

        if ch.is_alphabetic() {
            tokens.push(Token::Variable(Variable {
                name: ch.to_string(),
            }));
        }

        chars_left -= 1;
    }

    //tokens.pop();
    tokens
}

fn get_token(ch: char) -> Option<Token> {
    match ch {
        '+' => Some(Token::Operator(Operator::Add)),
        '-' => Some(Token::Operator(Operator::Substract)),
        '*' => Some(Token::Operator(Operator::Multiply)),
        '/' => Some(Token::Operator(Operator::Divide)),
        '^' => Some(Token::Operator(Operator::Exponentiate)),
        '(' => Some(Token::LeftParenthesis),
        ')' => Some(Token::RightParenthesis),
        ',' => Some(Token::Comma),
        _ => None,
    }
}

pub fn tokenize(expr: &str) -> TokenList {
    let trimmed = expr.replace(" ", "");
    let mut len = trimmed.len();
    let mut chars = trimmed.chars();

    let mut tokens = Vec::with_capacity(len);
    let mut temp = String::new();

    while let Some(c) = chars.next() {
        if c.is_alphanumeric() || c == '_' {
            temp.push(c);
            if len > 1 {
                len -= 1;
                continue;
            }
        }

        if !temp.is_empty() {
            if c == '(' {
                tokens.push(Token::Function(Function::new(temp.clone())));
                temp.clear();
                len -= 1;
                continue;
            }
            else {
                // TODO: maybe implement implicit multiplication
                tokens.append(&mut parse_implicit(&temp));
            }

            temp.clear();
        }

        if let Some(recognized_token) = get_token(c) {
            tokens.push(recognized_token);
        }

        len -= 1;
    }

    debug!("Tokens: {:?}", tokens);
    debug!("--------------------");

    tokens
}
