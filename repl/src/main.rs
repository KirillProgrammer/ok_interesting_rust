mod parsemath;

use parsemath::ast;
use parsemath::parser::{ParseError, Parser};

fn main() {
    println!("Hellof, welcome to Arithmetic expression evaluator!");
    println!("You can calculate value for expression such as 2*3+(4-5)+2^3/4 ");
    println!("Allowed numbers: positive, negative, decimals");
    println!("Supported operations: Add, Subtract, Multiply, Divide, PowerOf(^). Divide");
    println!("Enter your arithmetic expression bellow: ");

    loop {
        println!(">>>>>>>>>>>>>>>>>>>");
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => {
                        println!("Error in evaluating expression. Please enter a valid expression");
                    }
                };
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("Generated AST is {:?}", ast);
    Ok(ast::eval(ast)?)
}
