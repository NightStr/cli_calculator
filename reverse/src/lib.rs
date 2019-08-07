use regex::Regex;

#[derive(Debug)]
struct OperationToken {
    name: String,
}

impl OperationToken {
    pub fn new(name: String) -> OperationToken {
        OperationToken { name }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_priority(&self) -> Result<u8, &'static str> {
        return match &self.name {
            name if name == "+" || name == "-" => Ok(1),
            name if name == "*" || name == "/" => Ok(2),
            _ => Err("Priority if undefined"),
        };
    }

    pub fn execute(&self, op1: i64, op2: i64) -> Result<i64, String> {
        let value = match &self.name {
            name if name == "+" => op1.checked_add(op2),
            name if name == "-" => op1.checked_sub(op2),
            name if name == "*" => op1.checked_mul(op2),
            name if name == "/" => op1.checked_div(op2),
            name => {
                return Err(format!("Operation \"{}\"is not allowed", name));
            }
        };
        match value {
            Some(value) => Ok(value),
            None => Err(format!("{} {} {}. Overflow detected.", op1, self.name, op2)),
        }
    }
}

#[derive(Debug)]
struct NumberToken {
    value: i64,
}

impl NumberToken {
    pub fn new(value: i64) -> NumberToken {
        NumberToken { value }
    }
    pub fn get_value(&self) -> &i64 {
        &self.value
    }
}

#[derive(Debug)]
struct ParenthesisToken {
    t: TokenType,
}

impl ParenthesisToken {
    pub fn new_opened() -> ParenthesisToken {
        ParenthesisToken {
            t: TokenType::OpenedParenthesis,
        }
    }

    pub fn new_closed() -> ParenthesisToken {
        ParenthesisToken {
            t: TokenType::ClosedParenthesis,
        }
    }
    pub fn get_type(&self) -> TokenType {
        match &self.t {
            TokenType::OpenedParenthesis => TokenType::OpenedParenthesis,
            TokenType::ClosedParenthesis => TokenType::ClosedParenthesis,
            _ => panic!("Type of parenthesis is not allowed"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum TokenType {
    Operation,
    Number,
    OpenedParenthesis,
    ClosedParenthesis,
}

#[derive(Debug)]
struct Token {
    op: Option<OperationToken>,
    number: Option<NumberToken>,
    parenthesis: Option<ParenthesisToken>,
}

impl Token {
    pub fn new_number(number: i64) -> Token {
        Token {
            op: None,
            number: Some(NumberToken::new(number)),
            parenthesis: None,
        }
    }

    pub fn new_operation(name: String) -> Token {
        Token {
            op: Some(OperationToken::new(name)),
            number: None,
            parenthesis: None,
        }
    }

    pub fn new_open_parenthesis() -> Token {
        Token {
            op: None,
            number: None,
            parenthesis: Some(ParenthesisToken::new_opened()),
        }
    }

    pub fn new_closed_parenthesis() -> Token {
        Token {
            op: None,
            number: None,
            parenthesis: Some(ParenthesisToken::new_closed()),
        }
    }

    pub fn get_number(&self) -> Option<i64> {
        match &self.number {
            Some(n) => Some(*n.get_value()),
            _ => None,
        }
    }

    pub fn get_operation_name(&self) -> Option<&String> {
        match &self.op {
            Some(op) => Some(op.get_name()),
            _ => None,
        }
    }

    pub fn get_operation_priority(&self) -> Result<u8, &'static str> {
        match &self.op {
            Some(operation) => return operation.get_priority(),
            None => return Err("Token does not contain a operator"),
        };
    }

    pub fn get_type(&self) -> TokenType {
        if let Some(_) = self.op {
            return TokenType::Operation;
        }
        if let Some(_) = self.number {
            return TokenType::Number;
        }
        if let Some(p) = &self.parenthesis {
            return p.get_type();
        }
        panic!("Type of token is undefined");
    }

    pub fn execute(&self, op1: i64, op2: i64) -> Result<i64, String> {
        match &self.op {
            Some(op) => op.execute(op1, op2),
            None => Err("Toke is not operation".to_string()),
        }
    }
}

fn parse_token<'a, 'b>(regexp: &'a str, exp: &'b str) -> (Option<String>, &'b str) {
    let exp = exp;
    let re = Regex::new(regexp).unwrap();
    let captures = match re.captures(&exp[..]) {
        Some(v) => v,
        None => return (None, exp),
    };
    let captured = &captures[0];
    (Some(captured.to_string()), &exp[captured.len()..exp.len()])
}

fn number_parse(exp: &str) -> (Option<String>, &str) {
    parse_token(r"^(\d+)", exp)
}

fn operation_parse(exp: &str) -> (Option<String>, &str) {
    parse_token(r"^([+, \-, \*, //])", exp)
}

fn parentheses_parse(exp: &str) -> (Option<String>, &str) {
    parse_token(r"^([\(, \)])", exp)
}

fn tokenizing(exp: &String) -> Result<Vec<Token>, String> {
    let mut exp = exp.clone();
    let mut tokens: Vec<Token> = Vec::new();

    while exp.len() > 0 {
        if let (Some(n), new_exp) = number_parse(&exp[..]) {
            exp = new_exp.to_string();
            let number: i64 = match n.parse() {
                Ok(v) => v,
                Err(e) => return Err(format!("{} {}", n, e)),
            };
            tokens.push(Token::new_number(number));
        } else if let (Some(n), new_exp) = operation_parse(&exp[..]) {
            exp = new_exp.to_string();
            tokens.push(Token::new_operation(n));
        } else if let (Some(n), new_exp) = parentheses_parse(&exp[..]) {
            exp = new_exp.to_string();
            tokens.push(match &n[..] {
                "(" => Token::new_open_parenthesis(),
                ")" => Token::new_closed_parenthesis(),
                _ => panic!(format!("{} is not parenthesis", n)),
            });
        } else {
            return Err(format!(
                "Parsing error, expression: {} does not contain a valid token",
                exp
            ));
        }
    }
    return Ok(tokens);
}

fn parsing(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut previous_token_type = TokenType::Operation;
    let mut processed_tokens: Vec<Token> = Vec::new();
    let mut need_close_parentheses: bool = false;
    let mut pos = 0;
    for token in tokens {
        match token.get_type() {
            TokenType::Operation if token.get_operation_name().unwrap() == "-" => {
                match previous_token_type {
                    TokenType::Number | TokenType::ClosedParenthesis => {
                        processed_tokens.push(Token::new_operation("+".to_string()))
                    }
                    _ => {}
                }
                processed_tokens.push(Token::new_open_parenthesis());
                processed_tokens.push(Token::new_number(-1));
                processed_tokens.push(Token::new_operation("*".to_string()));
                need_close_parentheses = true;
            }
            TokenType::Operation => {
                match previous_token_type {
                    TokenType::OpenedParenthesis if token.get_operation_name().unwrap() == "+" => {}
                    TokenType::OpenedParenthesis => {
                        return Err(format!(
                        "Invalid expression. Operation {} is not allowed after opened parenthesis",
                        token.get_operation_name().unwrap()
                    ))
                    }
                    TokenType::Operation => {
                        return Err(format!(
                            "Invalid operation {} in position {}",
                            token.get_operation_name().unwrap(),
                            pos
                        ))
                    }
                    _ => processed_tokens.push(token),
                };
                previous_token_type = TokenType::Operation;
            }
            TokenType::Number => {
                processed_tokens.push(token);
                if need_close_parentheses {
                    need_close_parentheses = false;
                    processed_tokens.push(Token::new_closed_parenthesis());
                }
                previous_token_type = TokenType::Number;
            }
            TokenType::OpenedParenthesis => {
                processed_tokens.push(token);
                previous_token_type = TokenType::OpenedParenthesis;
            }
            TokenType::ClosedParenthesis => {
                if previous_token_type == TokenType::Operation {
                    return Err(format!("Invalid operation in position {}", pos));
                }
                processed_tokens.push(token);
                previous_token_type = TokenType::ClosedParenthesis;
            }
        }
        pos += 1;
    }
    if previous_token_type == TokenType::Operation {
        return Err("Invalid expression. Expression can't end on an operation".to_string());
    }
    return Ok(processed_tokens);
}

fn clear(exp: &String) -> String {
    return exp.replace(" ", "");
}

fn postfix(parsed_exp: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    let mut postfix_tokens: Vec<Token> = Vec::new();
    let mut operation_stack: Vec<Token> = Vec::new();

    for token in parsed_exp {
        match token.get_type() {
            TokenType::Number => {
                postfix_tokens.push(token);
            }
            TokenType::OpenedParenthesis => {
                operation_stack.push(token);
            }
            TokenType::ClosedParenthesis => loop {
                let last_operation = operation_stack.pop();
                match last_operation {
                    Some(operator) => {
                        if operator.get_type() == TokenType::OpenedParenthesis {
                            break;
                        } else {
                            postfix_tokens.push(operator);
                        }
                    }
                    None => return Err("An unclosed parenthesis was found."),
                };
            },
            TokenType::Operation if operation_stack.len() == 0 => operation_stack.push(token),
            TokenType::Operation => {
                let priority = token.get_operation_priority()?;

                while operation_stack.len() > 0 {
                    let last_token_operation = operation_stack.pop().unwrap();

                    match last_token_operation.get_type() {
                        TokenType::Operation => {
                            let last_token_priority =
                                last_token_operation.get_operation_priority()?;

                            if last_token_priority >= priority {
                                postfix_tokens.push(last_token_operation);
                            } else {
                                operation_stack.push(last_token_operation);
                                break;
                            }
                        }
                        TokenType::ClosedParenthesis | TokenType::OpenedParenthesis => {
                            operation_stack.push(last_token_operation);
                            break;
                        }
                        TokenType::Number => panic!("Number can't be in iperation stack"),
                    }
                }
                operation_stack.push(token);
            }
        }
    }
    while operation_stack.len() > 0 {
        postfix_tokens.push(operation_stack.pop().unwrap());
    }
    return Ok(postfix_tokens);
}

fn calculate(postfix_exp: Vec<Token>) -> Result<i64, String> {
    let mut result: Vec<i64> = Vec::new();
    for token in postfix_exp {
        match token.get_type() {
            TokenType::Number => result.push(token.get_number().unwrap()),
            TokenType::Operation => {
                let op2 = result.pop().unwrap();
                let op1 = result.pop().unwrap();
                result.push(token.execute(op1, op2)?)
            }
            _ => return Err("Found unexpected token".to_string()),
        }
    }
    match result.len() {
        0 => Err("No calculation result".to_string()),
        l if l > 1 => Err(format!("Fail evaluate a expression. Result: {:?}", result)),
        _ => Ok(result.pop().unwrap()),
    }
}

pub fn eval(exp: &String) -> Result<i64, String> {
    let mut tokens = tokenizing(&clear(exp))?;
    tokens = parsing(tokens)?;
    tokens = postfix(tokens)?;
    calculate(tokens)
}
