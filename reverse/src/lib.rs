use regex::Regex;

#[derive(Debug)]
pub struct OperationToken {
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
            name if name == "(" => Ok(0),
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
pub struct NumberToken {
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

#[derive(PartialEq)]
pub enum TokenType {
    Operation,
    Number,
}

#[derive(Debug)]
pub struct Token {
    op: Option<OperationToken>,
    number: Option<NumberToken>,
}

impl Token {
    pub fn new_number(number: i64) -> Token {
        Token {
            op: None,
            number: Some(NumberToken::new(number)),
        }
    }

    pub fn new_operation(name: String) -> Token {
        Token {
            op: Some(OperationToken::new(name)),
            number: None,
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

    pub fn get_type(&self) -> Option<TokenType> {
        match self.op {
            Some(_) => Some(TokenType::Operation),
            None => match self.number {
                Some(_) => Some(TokenType::Number),
                None => None,
            },
        }
    }
    pub fn execute(&self, op1: i64, op2: i64) -> Result<i64, String> {
        match &self.op {
            Some(op) => op.execute(op1, op2),
            None => Err("Toke is not operation".to_string()),
        }
    }
}

fn number_parse(exp: &str) -> (Option<String>, &str) {
    let exp = exp;
    let re = Regex::new(r"^(\d)").unwrap();
    let captures = match re.captures(&exp[..]) {
        Some(v) => v,
        None => return (None, exp),
    };
    let captured = &captures[0];
    (Some(captured.to_string()), &exp[captured.len()..exp.len()])
}

fn operation_parse(exp: &str) -> (Option<String>, &str) {
    let exp = exp;
    let re = Regex::new(r"^([+, \-, \*, //])").unwrap();
    let captures = match re.captures(&exp[..]) {
        Some(v) => v,
        None => return (None, exp),
    };
    let captured = &captures[0];
    (Some(captured.to_string()), &exp[captured.len()..exp.len()])
}

fn parentheses_parse(exp: &str) -> (Option<String>, &str) {
    let exp = exp;
    let re = Regex::new(r"^([\(, \)])").unwrap();
    let captures = match re.captures(&exp[..]) {
        Some(v) => v,
        None => return (None, exp),
    };
    let captured = &captures[0];
    (Some(captured.to_string()), &exp[captured.len()..exp.len()])
}

pub fn tokenizing(exp: &String) -> Result<Vec<Token>, String> {
    let mut exp = exp.clone();
    let mut tokens: Vec<Token> = Vec::new();

    while exp.len() > 0 {
        if let (Some(n), new_exp) = number_parse(&exp[..]) {
            exp = new_exp.to_string();
            tokens.push(Token::new_number(n.parse().unwrap()));
        } else if let (Some(n), new_exp) = operation_parse(&exp[..]) {
            exp = new_exp.to_string();
            tokens.push(Token::new_operation(n));
        } else if let (Some(n), new_exp) = parentheses_parse(&exp[..]) {
            exp = new_exp.to_string();
            tokens.push(Token::new_operation(n));
        } else {
            return Err(format!(
                "Parsing error, expression: {} does not contain a valid token",
                exp
            ));
        }
    }
    return Ok(tokens);
}

pub fn parsing(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut previous_token_type = TokenType::Operation;
    let mut processed_tokens: Vec<Token> = Vec::new();
    let mut need_close_parentheses: bool = false;
    for token in tokens {
        match token.get_type().unwrap() {
            TokenType::Operation => {
                if token.get_operation_name().unwrap() == "-"
                    && previous_token_type == TokenType::Operation
                {
                    processed_tokens.push(Token::new_operation("(".to_string()));
                    processed_tokens.push(Token::new_number(-1));
                    processed_tokens.push(Token::new_operation("*".to_string()));
                    need_close_parentheses = true;
                } else {
                    processed_tokens.push(token);
                }
                previous_token_type = TokenType::Operation;
            }
            TokenType::Number => {
                processed_tokens.push(token);
                if need_close_parentheses {
                    need_close_parentheses = false;
                    processed_tokens.push(Token::new_operation(")".to_string()));
                }
                previous_token_type = TokenType::Number;
            }
        }
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
        match token.get_type().unwrap() {
            TokenType::Number => postfix_tokens.push(token),
            TokenType::Operation => {
                let operation_name = token.get_operation_name().unwrap();
                if operation_stack.len() == 0 {
                    operation_stack.push(token);
                } else if operation_name == "(" {
                    operation_stack.push(token);
                } else if operation_name == ")" {
                    loop {
                        let last_operation = operation_stack.pop();
                        match last_operation {
                            Some(operator) => {
                                if operator.get_operation_name().unwrap() == "(" {
                                    break;
                                } else {
                                    postfix_tokens.push(operator);
                                }
                            }
                            None => return Err("An unclosed parenthesis was found."),
                        };
                    }
                } else {
                    let priority = token.get_operation_priority()?;

                    while operation_stack.len() > 0 {
                        let last_token_operation = operation_stack.pop().unwrap();
                        let last_token_priority = last_token_operation.get_operation_priority()?;

                        if last_token_priority >= priority {
                            postfix_tokens.push(last_token_operation);
                        } else {
                            operation_stack.push(last_token_operation);
                            break;
                        }
                    }
                    operation_stack.push(token);
                }
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
        match token.get_type().unwrap() {
            TokenType::Number => result.push(token.get_number().unwrap()),
            TokenType::Operation => {
                let op2 = result.pop().unwrap();
                let op1 = result.pop().unwrap();
                result.push(token.execute(op1, op2)?)
            }
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
    return calculate(tokens);
}
