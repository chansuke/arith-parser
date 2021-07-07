use arith_parser::eval;
use arith_parser::Parser;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let input = rl.readline("Please enter the arithmetric >> ");

        match evaluate(input.unwrap()) {
            Ok(val) => println!("Computed number is: {}", val),
            Err(_) => println!("Please enter valid expression"),
        };
    }
}

fn evaluate(expr: String) -> Result<f64, Box<dyn std::error::Error>> {
    let expr = expr.split_whitespace().collect::<String>();

    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;

    println!("The generated AST is {:?}", ast);

    eval(ast)
}
