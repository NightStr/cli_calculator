pub mod reverse {
    pub fn reverse(exp: &String) -> Result<i32, String> {
        let mut result: Option<i32> = None;
        let mut operation: Option<char> = None;
        
        for sub_str in exp.split_whitespace() {
            if let Ok(n) = sub_str.parse::<i32>() {
                if let Some(r) = result {
                    result = match operation {
                        Some(op) if op == '+' => Some(r + n),
                        Some(op) if op == '-' => Some(r - n),
                        _ => {
                            return Err(format!("Error, excepted operation, but recived {}", sub_str)); 
                        }
                    };
                    operation = None;
                } else {
                    result = Some(n);
                }
            } else if sub_str.len() == 1 {
                match sub_str {
                    _ if result == None => {
                        return Err(format!("First must be a number, not a operation"));
                    }
                    sub_str if ["+", "-"].contains(&sub_str) => operation = sub_str.chars().next(),
                    _ => {
                        return Err(format!("Operation {} is not allowed", sub_str));
                    }
                }
            } else {
                return Err(format!("{} is not allowed. Must be number or operation.", sub_str));
            }
        }
        return match result {
            Some(n) => Ok(n),
            None => Err("Result is undefined".to_string())
        };
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn one_plus_one() {
            let result = reverse(&"1 + 1".to_string());
            assert_eq!(result, Ok(2));
        }
        #[test]
        fn operation_first() {
            let result = reverse(&"+ 1 + 2".to_string());
            assert!(result.is_err(), format!("{:?}", result));
        }
        #[test]
        fn numbers_without_operations() {
            let result = reverse(&"1 2".to_string());
            assert!(result.is_err(), format!("{:?}", result));
        }
    }
}
