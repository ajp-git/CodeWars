use std::{collections::HashMap, fmt::format, thread::panicking};

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
    FunctionCall(String, Vec<f32>),
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
    expressions: Vec<Expr>,
    variables: HashMap<String, f32>,
    functions: HashMap<String, Expr>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            tokens: Vec::new(),
            expressions: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        self.tokenise(input)?;
        println!("Token struct {:?}", self.tokens);
        if let Ok(result) = self.parse() {
            println!("{:?}", result);
            let result = self.evaluate(&result);

            println!("Evaluated {:?}", result);
            return result;
        }
        Err("Parse not ok".to_string())
    }

    fn tokenise(&mut self, input: &str) -> Result<&mut Vec<Token>, String> {
        let mut in_number = false;
        let mut in_identifier = false;

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
                        if c.is_digit(10) || c == &'.' {
                            num.push(input_iter.next().unwrap());
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

        while !self.eof() {
            let res = self.parse_expr();
            println!("{:?}", res);
            result.push(res);
        }
        Ok(result[0].clone())
    }

    fn does_variable_exists(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
    fn eof(&mut self) -> bool {
        self.tokens.len() == 0
    }

    fn at(&self) -> Option<Token> {
        if self.tokens.len() > 0 {
            return Some(self.tokens[0].clone());
        }
        None
    }

    fn eat(&mut self) -> Token {
        let res = self.at();
        self.tokens = self.tokens[1..].to_vec();
        res.unwrap()
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut left = self.parse_multiplicative_expr();
        while let Some(Token::Operator(op)) = self.at() {
            match op {
                Operator::Add | Operator::Substract => {
                    let _ = self.eat();
                    let right = self.parse_multiplicative_expr();
                    left = Expr::BinaryOp(Box::new(left.clone()), op, Box::new(right));
                }
                _ => {
                    break;
                }
            }
        }
        left
    }
    fn parse_multiplicative_expr(&mut self) -> Expr {
        let mut left = self.parse_assignement_expr();
        while let Some(Token::Operator(op)) = self.at() {
            match op {
                Operator::Multiply | Operator::Divide | Operator::Modulo => {
                    let _ = self.eat();
                    let right = self.parse_assignement_expr();
                    left = Expr::BinaryOp(Box::new(left.clone()), op, Box::new(right));
                }
                _ => {
                    break;
                }
            }
        }
        left
    }

    fn parse_assignement_expr(&mut self) -> Expr {
        let mut left = self.parse_function_expr();
        while let Some(ass) = self.at() {
            match ass {
                Token::Assignement => match left {
                    Expr::Identifier(s) => {
                        let _ = self.eat();
                        let right = self.parse_function_expr();
                        left = Expr::Assignement(s, Box::new(right));
                    }
                    _ => {
                        panic!("Cannot assign something to a value");
                    }
                },
                _ => {
                    break;
                }
            }
        }
        left
    }

    fn parse_function_expr(&mut self) -> Expr {
        if self.at() == Some(Token::Fn) {
            let mut variables: Vec<String> = Vec::new();
            /* function be like fn avg x y => (x+y)/2 */
            self.eat();
            println!("function declaration");
            /* next token is function name */
            let fn_name: String = self.at().unwrap().get_string().unwrap();
            println!("function name is {:?}", fn_name);
            self.eat();
            /* next tokens should be variables, until FnOperator */
            while self.at() != Some(Token::FnOperator) {
                let var_name: String = self.at().unwrap().get_string().unwrap();
                print!("Variable for function {} ", var_name);
                variables.push(var_name);
                println!();
                self.eat();
            }
            /* We are at the fnoperator, Expr follows */
            self.eat(); // consume the fnoperator

            let fn_code_expr = self.parse_expr();

            let fn_expr =
                Expr::FunctionDefinition(fn_name.clone(), variables, Box::new(fn_code_expr));
            self.functions.insert(fn_name, fn_expr.clone());
            return fn_expr;
        } else {
            return self.parse_primary_expr();
        }
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let left = self.at().unwrap();
        match left {
            Token::Number(x) => {
                self.eat();
                return Expr::Number(x);
            }
            Token::Identifier(s) => {
                self.eat();
                if let Some(Token::Number(_)) = self.at() {
                    let mut variables: Vec<f32> = Vec::new();
                    while let Some(Token::Number(arg)) = self.at() {
                        variables.push(arg);
                        self.eat();
                    }
                    return Expr::FunctionCall(s, variables);
                } else {
                    return Expr::Identifier(s);
                }
            }
            Token::OpenParent => {
                self.eat();
                let result = self.parse_expr();
                self.eat();
                return result;
            }
            _ => panic!("Unexpected Token: {:?}", left),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Option<f32>, String> {
        match expr {
            Expr::Number(v) => {
                return Ok(Some(*v));
            }
            Expr::BinaryOp(e1, op, e2) => {
                if let Ok(v_e1) = self.evaluate(e1) {
                    if let Ok(v_e2) = self.evaluate(e2) {
                        let v_e1 = v_e1.unwrap();
                        let v_e2 = v_e2.unwrap();
                        match op {
                            Operator::Add => {
                                return Ok(Some(v_e1 + v_e2));
                            }
                            Operator::Substract => {
                                return Ok(Some(v_e1 - v_e2));
                            }
                            Operator::Divide => {
                                return Ok(Some(v_e1 / v_e2));
                            }
                            Operator::Multiply => {
                                return Ok(Some(v_e1 * v_e2));
                            }
                            Operator::Modulo => {
                                return Ok(Some(v_e1 % v_e2));
                            }
                        }
                    } else {
                        return Err(format!("Bad value for operation {:?}", expr));
                    }
                } else {
                    return Err(format!("Bad value for operation {:?}", expr));
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
                    return Ok(Some(v_e1));
                } else {
                    return Err(format!("Error evaluating expression {:?}", expr));
                }
            }
            Expr::Identifier(s) => {
                if let Some(val) = self.variables.get(s) {
                    return Ok(Some(*val));
                }
                return Err(format!("Variable or function {:?} not found", s));
            }

            Expr::FunctionCall(name, values) => {
                if let Some(function_called) = self.functions.get(name) {
                    if function_called.get_function_variables().len() == values.len() {
                        let variables = function_called.get_function_variables();
                        for (var, val) in variables.iter().zip(values) {
                            self.variables.insert(var.clone(), *val);
                        }
                        return self.evaluate(&function_called.get_function_code());
                    } else {
                        return Err("Bad args countfor function".to_string());
                    }
                } else {
                    return Err("Unknown function".to_string());
                }
            }

            Expr::FunctionDefinition(name, _, _) => {
                if self.variables.get(&name.clone()).is_some() {
                    return Err("Function cannot use already defined variable name".to_string());
                }
                Ok(None)
            }
            _ => panic!("Invalid operation {:?}", expr),
        }
    }
}

fn main() {
    let mut i = Interpreter::new();
    //assert_eq!(i.input("1 + 1"), Ok(Some(2.0)));
    //i.input("x = 13 + (y = 3)");
    i.input("fn avg x y => (x + y) / 2");
    i.input("avg 2 3");
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
