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
        match &self.name {
            name if name == "+" => Ok(op1 + op2),
            name if name == "-" => Ok(op1 - op2),
            name if name == "*" => Ok(op1 * op2),
            name if name == "/" => Ok(op1 / op2),
            name => Err(format!("Operation \"{}\"is not allowed", name)),
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

pub fn parse(exp: &String) -> Result<Vec<Token>, String> {
    let mut previous_token_type = TokenType::Operation;
    let mut tokens: Vec<Token> = Vec::new();
    let mut need_close = false;

    for ch in exp.chars() {
        match ch {
            '0'...'9' => {
                let current_number = ch.to_digit(10).unwrap() as i64;
                match previous_token_type {
                    TokenType::Operation => {
                        tokens.push(Token::new_number(current_number));
                        previous_token_type = TokenType::Number;
                    }
                    TokenType::Number => {
                        let last_token = tokens.pop();

                        match last_token {
                            Some(token) => match token.get_type().unwrap() {
                                TokenType::Number => {
                                    let number = token.get_number().unwrap();
                                    if number > i64::max_value() / 10 - current_number {
                                        return Err(
                                            "Overflow detected. Value is too big".to_string()
                                        );
                                    }
                                    tokens.push(Token::new_number(number * 10 + current_number));
                                }
                                _ => {
                                    tokens.push(token);
                                    tokens.push(Token::new_number(current_number));
                                }
                            },
                            None => {
                                tokens.push(Token::new_number(current_number));
                            }
                        };
                        previous_token_type = TokenType::Number;
                    }
                }
            }
            _ if ['+', '*', '/', '(', ')', '-'].contains(&ch) => {
                match previous_token_type {
                    TokenType::Operation if ch == '-' => {
                        tokens.push(Token::new_operation('('.to_string()));
                        tokens.push(Token::new_number(-1));
                        tokens.push(Token::new_operation('*'.to_string()));
                        need_close = true;
                    }
                    TokenType::Number if need_close == true => {
                        need_close = false;
                        tokens.push(Token::new_operation(')'.to_string()));
                        tokens.push(Token::new_operation(ch.to_string()));
                    }
                    _ => {
                        tokens.push(Token::new_operation(ch.to_string()));
                    }
                }
                previous_token_type = TokenType::Operation;
            }
            _ => {
                return Err(format!("Operation {} is not allowed", ch));
            }
        };
    }
    if need_close == true {
        tokens.push(Token::new_operation(')'.to_string()));
    }
    return Ok(tokens);
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
    if result.len() > 1 {
        Err(format!("Fail evaluate a expression. Result: {:?}", result))
    } else {
        Ok(result.pop().unwrap())
    }
}

pub fn eval(exp: &String) -> Result<i64, String> {
    let parsed_exp = parse(&clear(exp))?;
    let postfixed_exp = postfix(parsed_exp)?;
    return calculate(postfixed_exp);
}
