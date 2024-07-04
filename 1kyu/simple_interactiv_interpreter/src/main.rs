use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f32),
    Identifier(String),
    Operator(Operator),
    Fn,
    OpenParent,
    CloseParent,
    FnOperator,
    Assignement,
    FnCall(String),
    FunctionDefinition(String),
}

impl Token {
    fn get_string(&self) -> Option<String> {
        match self {
            Token::Identifier(s) => Some(s.to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Modulo,
}
#[derive(Debug, Clone)]
enum Expr {
    Number(f32),
    Identifier(String),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
    Assignement(String, Box<Expr>),
    FunctionCall(String, Box<Vec<Expr>>),
    FunctionDefinition(String, Vec<String>, Box<Expr>),
}
impl Expr {
    fn get_function_variables(&self) -> Vec<String> {
        match self {
            Expr::FunctionDefinition(_, vars, _) => vars.clone(),
            _ => panic!("unknown function variable"),
        }
    }
    fn get_function_code(&self) -> Box<Expr> {
        match self {
            Expr::FunctionDefinition(_, _, code) => code.clone(),
            _ => panic!("unknown function code"),
        }
    }
}
struct Interpreter {
    tokens: Vec<Token>,
    variables: HashMap<String, f32>,
    functions: HashMap<String, Expr>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            tokens: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        if input.is_empty() {
            return Ok(None);
        }
        self.tokenise(input)?;
        if self.tokens.is_empty() {
            return Ok(None);
        }

        if self.tokens.len() > 1
            && self.tokens.len()
                == self
                    .tokens
                    .iter()
                    .filter(|t| matches!(t, Token::Number(_)))
                    .count()
        {
            return Err("Invalid input".to_string());
        }

        if let Ok(result) = self.parse() {
            let result = self.evaluate(&result);
            if self.at().is_some() {
                return Err("Too many arguments".to_string());
            }
            if result.is_ok() && self.at().is_some() {
                return Err("Invalid input".to_string());
            }
            return result;
        }
        Err("Parse not ok".to_string())
    }

    fn tokenise(&mut self, input: &str) -> Result<&mut Vec<Token>, String> {
        let mut input_iter = input.chars().peekable();

        let mut ops: HashMap<char, Operator> = HashMap::new();
        [
            ('+', Operator::Add),
            ('-', Operator::Substract),
            ('*', Operator::Multiply),
            ('/', Operator::Divide),
            ('%', Operator::Modulo),
        ]
        .iter()
        .for_each(|(k, v)| {
            ops.insert(*k, *v);
        });

        while let Some(c) = input_iter.next() {
            match c {
                '0'..='9' => {
                    let mut num = String::new();
                    num.push(c);
                    while let Some(c) = input_iter.peek() {
                        if c.is_ascii_digit() || c == &'.' {
                            num.push(input_iter.next().unwrap());
                        } else if c.is_ascii_alphabetic() {
                            return Err("Invalid char in number".to_string());
                        } else {
                            break;
                        }
                    }
                    self.tokens.push(Token::Number(num.parse::<f32>().unwrap()));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    ident.push(c);
                    while let Some(c) = input_iter.peek() {
                        if c.is_alphanumeric() || c == &'_' {
                            ident.push(input_iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if ident == "fn" {
                        self.tokens.push(Token::Fn);
                    } else if let Some(Token::Fn) = self.tokens.last() {
                        self.tokens.pop();
                        self.tokens.push(Token::FunctionDefinition(ident));
                    } else if self.functions.iter().filter(|(k, _)| k == &&ident).count() > 0 {
                        self.tokens.push(Token::FnCall(ident));
                    } else {
                        self.tokens.push(Token::Identifier(ident));
                    }
                }
                '(' => self.tokens.push(Token::OpenParent),

                ')' => self.tokens.push(Token::CloseParent),

                '+' | '-' | '*' | '/' | '%' => {
                    self.tokens.push(Token::Operator(*ops.get(&c).unwrap()));
                }

                '=' => {
                    if let Some(c) = input_iter.peek() {
                        if c == &'>' {
                            input_iter.next();
                            self.tokens.push(Token::FnOperator);
                        } else {
                            self.tokens.push(Token::Assignement);
                        }
                    }
                }

                ' ' | '\n' => {}
                _ => {
                    let res = format!("Unknown char {}", c);
                    return Err(res);
                }
            }
        }
        Ok(&mut self.tokens)
    }

    fn parse(&mut self) -> Result<Expr, String> {
        let mut result: Vec<Expr> = Vec::new();

        if !self.eof() {
            let res = self.parse_expr();
            if res.is_err() {
                return res;
            }
            if self.at().is_some() {
                return Err("Incorrect input parsed".to_string());
            }
            result.push(res.clone().unwrap());
        }
        Ok(result[0].clone())
    }

    fn eof(&mut self) -> bool {
        self.tokens.is_empty()
    }

    fn at(&self) -> Option<Token> {
        if !self.tokens.is_empty() {
            return Some(self.tokens[0].clone());
        }
        None
    }

    fn eat(&mut self) -> Token {
        let res = self.at();
        self.tokens = self.tokens[1..].to_vec();
        res.unwrap()
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_additive_expr()
    }
    fn parse_additive_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative_expr();
        if left.is_err() {
            return left;
        }
        while let Some(Token::Operator(op)) = self.at() {
            match op {
                Operator::Add | Operator::Substract => {
                    let _ = self.eat();
                    let right = self.parse_multiplicative_expr();
                    if right.is_err() {
                        return right;
                    }
                    left = Ok(Expr::BinaryOp(
                        Box::new(left.unwrap().clone()),
                        op,
                        Box::new(right.unwrap()),
                    ));
                }
                _ => {
                    break;
                }
            }
        }
        left
    }
    fn parse_multiplicative_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_assignement_expr();
        while let Some(Token::Operator(op)) = self.at() {
            match op {
                Operator::Multiply | Operator::Divide | Operator::Modulo => {
                    let _ = self.eat();
                    let right = self.parse_assignement_expr();
                    if right.is_err() {
                        return right;
                    }
                    left = Ok(Expr::BinaryOp(
                        Box::new(left.unwrap().clone()),
                        op,
                        Box::new(right.unwrap()),
                    ));
                }
                _ => {
                    break;
                }
            }
        }
        left
    }
    fn parse_assignement_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_function_expr();
        if left.is_err() {
            return left;
        }

        while let Some(ass) = self.at() {
            match ass {
                Token::Assignement => match left.unwrap() {
                    Expr::Identifier(s) => {
                        let _ = self.eat();
                        let right = self.parse_expr();
                        if right.is_err() {
                            return right;
                        } // Use parse_expr instead of parse_function_expr
                        left = Ok(Expr::Assignement(s, Box::new(right.unwrap())));
                    }
                    _ => {
                        return Err("Cannot assign something to a value".to_string());
                    }
                },
                _ => {
                    break;
                }
            }
        }
        left
    }
    fn parse_function_expr(&mut self) -> Result<Expr, String> {
        if let Some(Token::FunctionDefinition(fn_name)) = self.at() {
            let mut variables: Vec<String> = Vec::new();
            self.eat();

            while self.at() != Some(Token::FnOperator) {
                let var_name: String = self.at().unwrap().get_string().unwrap();
                variables.push(var_name);
                self.eat();
            }

            self.eat();

            let fn_code_expr = self.parse_expr();
            if fn_code_expr.is_err() {
                return fn_code_expr;
            }
            let unique_variables: HashSet<_> = variables.iter().collect();
            if variables.len() != unique_variables.len() {
                return Err("Duplicate variables name".to_string());
            }

            let mut identifiers: HashSet<String> = HashSet::new();

            self.collect_identifiers(&fn_code_expr.clone().unwrap(), &mut identifiers);

            let tot_variables: HashSet<String> = variables
                .iter()
                .chain(identifiers.iter())
                .cloned()
                .collect();
            if tot_variables.len() != identifiers.len() {
                return Err("Incorrect variables".to_string());
            }
            let fn_expr = Expr::FunctionDefinition(
                fn_name.clone(),
                variables,
                Box::new(fn_code_expr.unwrap()),
            );

            self.functions.insert(fn_name, fn_expr.clone());
            Ok(fn_expr)
        } else {
            self.parse_primary_expr()
        }
    }
    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        let left = self.at();
        if left.is_none() {
            return Err("missing token".to_string());
        }
        let left = left.unwrap();
        match left {
            Token::Number(x) => {
                self.eat();
                Ok(Expr::Number(x))
            }
            Token::Identifier(s) => {
                self.eat();

                Ok(Expr::Identifier(s))
            }
            Token::OpenParent => {
                self.eat();
                if let Some(Token::FunctionDefinition(_)) = self.at() {
                    return Err("Cannot define function after opening parentesis".to_string());
                }
                let result = self.parse_expr();
                self.eat();
                result
            }
            Token::FnCall(fn_name) => {
                self.eat();

                let called_function = match self.functions.get(&fn_name) {
                    Some(func) => func,
                    None => return Err("Bad function name".to_string()),
                };
                let expected_arg_count = called_function.get_function_variables().len();

                let mut arguments = Vec::new();
                for _ in 0..expected_arg_count {
                    let res = self.parse_expr()?;
                    arguments.push(res);
                }
                if arguments.len() < expected_arg_count {
                    return Err(format!("Not enough arguments for function {}", fn_name));
                }

                Ok(Expr::FunctionCall(fn_name, Box::new(arguments)))
            }
            _ => Err(format!("Unexpected Token: {:?}", left)),
        }
    }
    fn collect_identifiers(&self, expr: &Expr, identifiers: &mut HashSet<String>) {
        match expr {
            Expr::Identifier(name) => {
                identifiers.insert(name.clone());
            }
            Expr::BinaryOp(left, _, right) => {
                self.collect_identifiers(left, identifiers);
                self.collect_identifiers(right, identifiers);
            }
            Expr::Assignement(name, expr) => {
                identifiers.insert(name.clone());
                self.collect_identifiers(expr, identifiers);
            }
            Expr::FunctionCall(_, args) => {
                for arg in args.iter() {
                    self.collect_identifiers(arg, identifiers);
                }
            }
            Expr::FunctionDefinition(_, params, body) => {
                for param in params {
                    identifiers.insert(param.clone());
                }
                self.collect_identifiers(body, identifiers);
            }
            _ => {}
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Option<f32>, String> {
        match expr {
            Expr::Number(v) => Ok(Some(*v)),
            Expr::BinaryOp(e1, op, e2) => {
                if let Ok(v_e1) = self.evaluate(e1) {
                    if let Ok(v_e2) = self.evaluate(e2) {
                        let v_e1 = v_e1.unwrap();
                        let v_e2 = v_e2.unwrap();
                        match op {
                            Operator::Add => Ok(Some(v_e1 + v_e2)),
                            Operator::Substract => Ok(Some(v_e1 - v_e2)),
                            Operator::Divide => Ok(Some(v_e1 / v_e2)),
                            Operator::Multiply => Ok(Some(v_e1 * v_e2)),
                            Operator::Modulo => Ok(Some(v_e1 % v_e2)),
                        }
                    } else {
                        Err(format!("Bad value for operation {:?}", expr))
                    }
                } else {
                    Err(format!("Bad value for operation {:?}", expr))
                }
            }
            Expr::Assignement(var, expr) => {
                if let Ok(v_e1) = self.evaluate(expr) {
                    let v_e1 = v_e1.unwrap();
                    if self.functions.get(var).is_some() {
                        return Err(format!(
                            "Cannot create a variable with a function named {:?}",
                            var
                        ));
                    }
                    self.variables.insert(var.clone(), v_e1);
                    Ok(Some(v_e1))
                } else {
                    Err(format!("Error evaluating expression {:?}", expr))
                }
            }
            Expr::Identifier(s) => {
                if let Some(val) = self.variables.get(s) {
                    return Ok(Some(*val));
                }
                if let Some(function_called) = self.functions.get(s) {
                    return self.evaluate(&function_called.get_function_code());
                }
                Err(format!("Variable or function {:?} not found", s))
            }

            Expr::FunctionCall(name, values) => {
                let function_called = if let Some(f) = self.functions.get(name).cloned() {
                    f
                } else {
                    return Err("Unknown function".to_string());
                };

                if function_called.get_function_variables().len() == values.len() {
                    let variables = function_called.get_function_variables();
                    let mut local_variables = HashMap::new();
                    for (var, val_expr) in variables.iter().zip(values.as_ref()) {
                        if let Ok(Some(val)) = self.evaluate(val_expr) {
                            local_variables.insert(var.clone(), val);
                        } else {
                            return Err(format!("Error evaluating argument {:?}", val_expr));
                        }
                    }
                    // Save the current state of variables
                    let old_variables = std::mem::replace(&mut self.variables, local_variables);
                    // Evaluate the function body with the new scope
                    let result = self.evaluate(&function_called.get_function_code());
                    // Restore the original variables
                    self.variables = old_variables;
                    result
                } else {
                    Err("Incorrect number of arguments for function".to_string())
                }
            }
            Expr::FunctionDefinition(name, _, _) => {
                if self.variables.get(&name.clone()).is_some() {
                    return Err("Function cannot use already defined variable name".to_string());
                }
                Ok(None)
            }
        }
    }
}

fn main() {
    let mut i = Interpreter::new();
    i.input("x = 23");
    i.input("y = 25");
    i.input("z = 0");
    i.input("fn one => 1");
    i.input("fn avg x y => (x + y) / 2");
    assert_eq!(i.input("one"), Ok(Some(1.0)));
}

#[test]
fn it_should_throw_an_error_when_function_contains_contains_duplicate_arguments() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 23"), Ok(Some(23.0)));
    assert_eq!(i.input("y = 25"), Ok(Some(25.0)));
    assert_eq!(i.input("z = 0"), Ok(Some(0.0)));
    assert!(i.input("fn add x x => x + x").is_err());
}

#[test]
fn it_should_throw_an_error_when_function_contains_invalid_variables() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 23"), Ok(Some(23.0)));
    assert_eq!(i.input("y = 25"), Ok(Some(25.0)));
    assert_eq!(i.input("z = 0"), Ok(Some(0.0)));
    assert!(i.input("fn add x y => x + z").is_err());
}

#[test]
fn it_should_throw_an_error_when_function_is_declared_within_an_expression() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 23"), Ok(Some(23.0)));
    assert_eq!(i.input("y = 25"), Ok(Some(25.0)));
    assert_eq!(i.input("z = 0"), Ok(Some(0.0)));
    assert!(i.input("(fn f => 1)").is_err());
}

#[test]
fn basic_arithmetic() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("1 + 1"), Ok(Some(2.0)));
    assert_eq!(i.input("2 - 1"), Ok(Some(1.0)));
    assert_eq!(i.input("2 * 3"), Ok(Some(6.0)));
    assert_eq!(i.input("8 / 4"), Ok(Some(2.0)));
    assert_eq!(i.input("7 % 4"), Ok(Some(3.0)));
}

#[test]
fn variables() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 1"), Ok(Some(1.0)));
    assert_eq!(i.input("x"), Ok(Some(1.0)));
    assert_eq!(i.input("x + 3"), Ok(Some(4.0)));
    assert!(i.input("y").is_err());
}

#[test]
fn functions() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("fn avg x y => (x + y) / 2"), Ok(None));
    assert_eq!(i.input("avg 4 2"), Ok(Some(3.0)));
    assert!(i.input("avg 7").is_err());
    assert!(i.input("avg 7 2 4").is_err());
}

#[test]
fn conflicts() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 1"), Ok(Some(1.0)));
    assert_eq!(i.input("fn avg x y => (x + y) / 2"), Ok(None));
    assert!(i.input("fn x => 0").is_err());
    assert!(i.input("avg = 5").is_err());
}

#[test]
fn reals() {
    let mut i = Interpreter::new();
    i.input("x = 23");
    i.input("y = 25");
    i.input("z = 0");
    i.input("fn one => 1");
    i.input("fn avg x y => (x + y) / 2");
    assert_eq!(i.input("one"), Ok(Some(1.0)));
}
#[test]
fn it_should_call_chained_functions() {
    let mut i = Interpreter::new();
    i.input("x = 23");
    i.input("y = 25");
    i.input("z = 0");
    i.input("fn avg x y => (x + y) / 2");
    i.input("fn echo x => x");
    assert_eq!(i.input("avg echo 4 echo 2"), Ok(Some(3.0)));
}
#[test]
fn it_should_continue_to_function_after_an_error_was_thrown() {
    let mut i = Interpreter::new();
    i.input("x = 7");
    i.input("y");
    i.input("y = x + 5");
    assert_eq!(i.input("y"), Ok(Some(12.0)));
}
#[test]
fn reals2() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("9"), Ok(Some(9.0)));
    assert!(i.input("1one").is_err());
}
#[test]
fn reals3() {
    let mut i = Interpreter::new();
    i.input("x = 23");
    i.input("y = 25");
    i.input("z = 0");
    i.input("fn avg x y => (x + y) / 2");
    i.input("fn echo x => x");
    assert!(i.input("avg echo 7 echo 2 echo 4").is_err());
}
