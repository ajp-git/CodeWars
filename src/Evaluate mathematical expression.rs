#[derive(Clone, Debug)]
enum Item {
    Op(char),
    Val(f64),
}

#[derive(Clone, Debug)]
struct Com {
    item: Item,
}

#[derive(Clone, Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    /*
    fn at(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }
     */
}
fn parse_items(expr: &str, in_stack: &mut Stack<Com>) -> Stack<Com> {
    let mut val: String = String::new();
    //    let mut in_val = false;
    let mut is_neg_val = false;
    let mut start = true;
    let mut last_sign = ' ';
    let mut previous_sign = ' ';
    //    let mut last_was_op = true;
    //    let mut double_sign = false;
    //    let mut last_sign: char = ' ';

    for c in expr.chars().into_iter() {
        println!("before {} in_stack: {:?}", c, in_stack);
        match c {
            '0'..='9' | '.' => {
                if last_sign == '-'
                    && (previous_sign == '+'
                        || previous_sign == '-'
                        || previous_sign == '*'
                        || previous_sign == '/'
                        || previous_sign == '('
                        || previous_sign == ')'
                        || start == true)
                {
                    in_stack.pop();
                    val.push('-');
                    last_sign = ' ';
                    previous_sign = ' ';
                }
                val.push(c);
                last_sign = ' ';
                start = false;
            }
            '+' | '-' | '*' | '/' | '(' | ')' => {
                if !val.is_empty() {
                    let number: Result<f64, _> = val.parse();

                    if let Ok(parsed_number) = number {
                        in_stack.push(Com {
                            item: Item::Val(parsed_number),
                        });
                        val.clear();
                    }
                }
                in_stack.push(Com { item: Item::Op(c) });
                previous_sign = last_sign;
                last_sign = c;
            }

            ' ' => {
                //                in_val = false;
            }
            _ => panic!("Invalid expression :"),
        }
    }
    if val.len() > 0 {
        let number: Result<f64, _> = val.parse();
        if let Ok(parsed_number) = number {
            in_stack.push(Com {
                item: Item::Val(parsed_number),
            });
        }
    }
    println!("Expression: {:?}", expr);
    println!("Input stack: {:?}", in_stack);
    in_stack.clone()
}

fn calc(expr: &str) -> f64 {
    let mut op_stack: Stack<char> = Stack::new();
    let mut out_stack: Stack<Com> = Stack::new();
    let mut temp_out_stack: Stack<Com> = Stack::new();
    let mut in_stack: Stack<Com> = Stack::new();

    println!("");
    println!("----------------------------------------------------------------");
    println!("{:?}", expr);
    println!("");
    in_stack = parse_items(expr, &mut in_stack);
    for item in &in_stack.items {
        match item.item {
            //- a number:
            //put it into the output queue
            Item::Val(v) => out_stack.push(Com { item: Item::Val(v) }),
            Item::Op(o) => match o {
                '*' | '/' => {
                    // 7.Push the current operator onto the stack
                    while let Some(&c) = op_stack.peek() {
                        if c == '*' || c == '/' {
                            //6. Pop operators from the stack onto the output queue
                            out_stack.push(Com {
                                item: Item::Op(op_stack.pop().unwrap()),
                            })
                        } else {
                            break;
                        }
                    }
                    op_stack.push(o);
                }
                '+' | '-' => {
                    //5. While there's an operator on the top of the stack with greater precedence:
                    while let Some(&c) = op_stack.peek() {
                        if c == '*' || c == '/' {
                            //6. Pop operators from the stack onto the output queue
                            out_stack.push(Com {
                                item: Item::Op(op_stack.pop().unwrap()),
                            })
                        } else {
                            break;
                        }
                    }

                    op_stack.push(o);
                }
                '(' => {
                    //        push it onto the operator stack
                    op_stack.push(o);
                }
                // Right parenthesis
                ')' => {
                    while let Some(&c) = op_stack.peek() {
                        //while the operator at the top of the operator stack is not a left parenthesis:
                        if c == '(' {
                            op_stack.pop();
                            break;
                        }
                        //  pop the operator from the operator stack into the output queue
                        let t = op_stack.pop();
                        if t.is_some() {
                            out_stack.push(Com {
                                item: Item::Op(t.unwrap()),
                            });
                        }
                    }
                    //pop the left parenthesis from the operator stack and discard it
                    // if there is a function token at the top of the operator stack, then:

                    //pop the function from the operator stack into the output queue
                    if let Some(c) = op_stack.peek() {
                        match c {
                            '+' | '-' | '*' | '/' => out_stack.push(Com {
                                item: Item::Op(op_stack.pop().unwrap()),
                            }),
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
        }
    }

    while let Some(&c) = op_stack.peek() {
        out_stack.push(Com {
            item: Item::Op(op_stack.pop().unwrap()),
        });
    }

    let mut val: Vec<f64> = Vec::new();
    println!("Output : {:?}", out_stack);

    if out_stack.items.len() == 1 {
        if let Some(com) = out_stack.pop() {
            if let Item::Val(val) = com.item {
                return val;
            }
        }
    }
    /*
        // Handle --
        let mut minus_count = 0;

        for c in out_stack.items.iter() {
            println!("\tHandling at {:?}", c);
            match c.item {
                Item::Op(op) => match op {
                    '-' => {
                        minus_count += 1;
                        if minus_count == 2 {
                            temp_out_stack.pop();
                            temp_out_stack.push(Com {
                                item: Item::Op('+'),
                            });
                            minus_count = 0;
                        } else {
                            temp_out_stack.push(Com {
                                item: Item::Op('-'),
                            });
                        }
                    }
                    _ => {
                        minus_count = 0;
                        temp_out_stack.push(Com { item: Item::Op(op) });
                    }
                },
                Item::Val(v) => {
                    temp_out_stack.push(Com { item: Item::Val(v) });
                    minus_count = 0;
                }
            }
        }
        println!("--------------------------------");
        println!("Cleaned out stack : {:?}", temp_out_stack);
        for c in temp_out_stack.items.iter() {
    */
    for c in out_stack.items.iter() {
        println!("\tLooking at {:?}", c);
        match c.item {
            Item::Op(op) => match op {
                '+' => {
                    if val.len() == 1 {
                        let v1 = val.pop().unwrap();
                        println!("\t\tEvaluating '+' but one value :{}", v1);
                        val.push(v1);
                        break;
                    }
                    let v1 = val.pop().unwrap();
                    let v2 = val.pop().unwrap();
                    println!("\t\tEvaluating {} + {}  :{}", v2, v1, v1 + v2);
                    val.push(v2 + v1)
                }
                '-' => {
                    if val.len() == 1 {
                        let v1 = val.pop().unwrap();
                        println!("\t\tEvaluating '-' but one value :{} => {}", v1, -v1);
                        val.push(-v1);
                        break;
                    }
                    let v1 = val.pop().unwrap();
                    let v2 = val.pop().unwrap();
                    println!("\t\tEvaluating {} - {}  :{}", v2, v1, v2 - v1);
                    val.push(v2 - v1)
                }
                '/' => {
                    let v1 = val.pop().unwrap();
                    let v2 = val.pop().unwrap();
                    println!("\t\tEvaluating {} / {}  :{}", v2, v1, v2 / v1);
                    val.push(v2 / v1)
                }
                '*' => {
                    let v1 = val.pop().unwrap();
                    let v2 = val.pop().unwrap();
                    println!("\t\tEvaluating {} * {}  :{}", v2, v1, v2 * v1);
                    val.push(v2 * v1)
                }
                _ => {}
            },
            Item::Val(v) => val.push(v),
        }
    }
    let tval = val.pop().unwrap();
    println!("\tResult :{}", tval);
    tval
}

#[cfg(test)]
mod tests {
    use super::calc;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        };
    }

    #[test]
    fn ajdev_values() {
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }
    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("-42", -42.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("((80 - (19)))", 61.0);
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("1 -(-(-4))", -3.0);
        assert_expr_eq!("1 - (4)", -3.0);
        assert_expr_eq!("1 - -4)", 5.0);
        assert_expr_eq!("1 - (-4)", 5.0);
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
    }
}
