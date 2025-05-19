use std::{io};

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Lpar,
    Rpar,
    Mystery
}

impl Operator {
    fn precedence(&self) -> Option<i32>{
        match self {
            Operator::Add | Operator::Sub => Some(0),
            Operator::Mult | Operator::Div => Some(1),
            _ => None
        }
    }
}

#[derive(PartialEq)]
enum State {
    ReadNumber,
    Operator(Operator),
}


fn main() {
    println!("Please input the expression you want to calculate the result of:)");

    loop {
        println!("Please input your expression:");

        let mut expr = String::new();

        io::stdin()
            .read_line(&mut expr)
            .expect("Failed to read line");

        let mut num_buf = String::new();
        let mut num_stack: Vec<i32> = Vec::new();
        let mut op_stack:Vec<Operator> = Vec::new();
        let mut state = State::ReadNumber;
        let operators = "+-*/()";
        let mut current_op:Operator;
        for c in expr.chars(){
            if c.is_digit(10) {
                num_buf.push(c);
                state = State::ReadNumber;
            }
            else if operators.contains(c){
                state = match c {
                        '+' => State::Operator(Operator::Add),
                        '-' => State::Operator(Operator::Sub),
                        '*' => State::Operator(Operator::Mult),
                        '/' => State::Operator(Operator::Div),
                        '(' => State::Operator(Operator::Lpar),
                        ')' => State::Operator(Operator::Rpar),
                        _ =>   State::Operator(Operator::Mystery),
                    };
                //this means we just stopped reading a number, so we
                //have to process the number and put it into the num_stack
                //one exception is a Lpar, so we have to check if there is
                //anything to put in
                if !num_buf.is_empty(){
                        num_stack.push(num_buf.parse::<i32>().unwrap());
                        num_buf.clear();
                    }
                current_op = match state {
                    State::Operator(sort) => sort,
                    _ => panic!(),
                };

                while op_stack.is_empty() ||
                        *op_stack.last().unwrap() == Operator::Lpar ||
                        current_op.precedence().unwrap() == op_stack.last().unwrap().precedence().unwrap()
                    {

                    }
            }
            else{
            }
            /* match c {
                content if content.is_digit(10) => num_buf.push(content),
                '+' | '-' | '*' | '/' | '(' | ')' => {op_stack.push(match c {
                        '+' => Operator::Add,
                        '-' => Operator::Sub,
                        '*' => Operator::Mult,
                        '/' => Operator::Div,
                        '(' => Operator::Lpar,
                        ')' => Operator::Rpar,
                        _ => Operator::Mystery
                    });
                    if !num_buf.is_empty(){
                        num_stack.push(num_buf.parse::<i32>().unwrap());
                        num_buf.clear();
                    }
                },
                _ => {
                    if !num_buf.is_empty(){
                        num_stack.push(num_buf.parse::<i32>().unwrap());
                        num_buf.clear();
                    }
                }
            } */
        }
        for num in num_stack{
            print!("{num} ")
        }
        print!("\n");
        for op in op_stack{
            print! ("{:?} ", op);
        }
        print!("\n");
    }
}
