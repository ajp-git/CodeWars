use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Token {
    Number(f32),
    Identifier(String),
    Operator(Operator),
    Fn,
    OpenParent,
    CloseParent,
    FnOperator,
    Assignement,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Modulo,
}

enum Expr {
    Number(f32),
    Identifier(String),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
    Assignement(String,Box<Expr>),
    FunctionCall(String,Vec<Expr>),
    FunctionDefinition(String, Vec<String>, Box<Expr> ),
}
struct Interpreter {
    tokens: Vec<Token>,
    expressions:Vec<Expr>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter { tokens: Vec::new(), expressions:Vec::new() }
    }

    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        self.tokenise(input)?;
        if let Ok(result)=self.parse() {
            let result = self.evaluate();

            println!("{:?}", result);
            return Ok(Some(0f32))    
        }
        Err("Parse not ok".to_string())
    }

    fn tokenise(&mut self, input: &str) -> Result<&mut Vec<Token>, String> {
        let mut in_number=false;
        let mut in_identifier = false;

        let mut input_iter=input.chars().peekable();

        let mut ops:HashMap<char,Operator>=HashMap::new();
        [('+',Operator::Add),
        ('-', Operator::Substract),
        ('*', Operator::Multiply),
        ('/', Operator::Divide),
        ('%', Operator::Modulo),
        ].iter().for_each(|(k,v)|{ops.insert(*k, *v);});

        while let Some(c) = input_iter.next() {

            match c {
                '0'..='9' =>{
                    let mut num=String::new();
                    num.push(c);
                    while let Some(c)=input_iter.peek() {
                        if c.is_digit(10) || c==&'.' {
                            num.push(input_iter.next().unwrap());
                        } else {
                            break;
                        }                        
                    }
                    self.tokens.push(Token::Number(num.parse::<f32>().unwrap()));
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident=String::new();
                    ident.push(c);
                    while let Some(c) = input_iter.peek() {
                        if c.is_alphanumeric() || c==&'_' {
                            ident.push(input_iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if ident=="fn" {
                        self.tokens.push(Token::Fn);
                    } else {
                        self.tokens.push(Token::Identifier(ident));                        
                    }
                },
                '(' => self.tokens.push(Token::OpenParent),
                
                ')' => self.tokens.push(Token::CloseParent),

                '+' | '-' | '*' | '/' | '%' =>{
                   self.tokens.push(Token::Operator(*ops.get(&c).unwrap()));
                },

                '=' => {
                    if let Some(c) = input_iter.peek() {
                        if c==&'>' {
                            input_iter.next();
                            self.tokens.push(Token::FnOperator);
                        } else {
                            self.tokens.push(Token::Assignement);
                        }
                    }
                }

                ' ' | '\n' => { },
            _ => {
                    let res=format!("Unknown char {}",c);
                    return Err(res);
                },     
            }
        }
        Ok(&mut self.tokens)
    }

    fn parse(&mut self) -> Result<(Expr, &[Token]), String> {
        self.parse_term(&self.tokens)
    }


    fn parse_term<'a>(&'a self, tokens: &'a[Token]) -> Result<(Expr, &[Token]), String> {
        let mut current_tokens=tokens;
        while let Some(token) = current_tokens.first() {
            match token {
                Token::Number(v) => {
                    return Ok((Expr::Number(*v),&current_tokens[1..]));
                },
                _ => { return Err("Not handled".to_string()) },
            }
        }
        Err("Bad parse end".to_string())
    }

    fn evaluate (&self) -> Result<Option<f32>, String> {

        Ok(Some(0.0))
    }
}

fn main(){
    let mut i = Interpreter::new();
    //assert_eq!(i.input("1 + 1"), Ok(Some(2.0)));
    i.input("1");
//    i.input("fn avg x y => (x + y) / 2");
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