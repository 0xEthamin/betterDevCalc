use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::process::Command;


#[derive(Debug, PartialEq)]
enum Operation 
{
    Add,
    Subtract,
    Multiply,
    OpenParen,
    CloseParen,
}

impl Operation 
{
    fn precedence(&self) -> u8
    {
        match self 
        {
            Operation::Add | Operation::Subtract => 1,
            Operation::Multiply => 2,
            Operation::OpenParen | Operation::CloseParen => 0,
        }
    }

    fn from_char(c: char) -> Result<Self, String> 
    {
        match c 
        {
            '+' => Ok(Operation::Add),
            '-' => Ok(Operation::Subtract),
            '*' => Ok(Operation::Multiply),
            '(' => Ok(Operation::OpenParen),
            ')' => Ok(Operation::CloseParen),
            _ => Err(format!("Invalid operator: {}", c)),
        }
    }

    fn apply(&self, left: i64, right: i64) -> i64 
    {
        match self 
        {
            Operation::Add => left + right,
            Operation::Subtract => left - right,
            Operation::Multiply => left * right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Base 
{
    Decimal,
    Hexadecimal,
}

impl Base 
{
    fn from_char(c: char) -> Result<Self, String> 
    {
        match c 
        {
            'd' => Ok(Base::Decimal),
            'h' => Ok(Base::Hexadecimal),
            _ => Err(format!("Invalid base: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Number 
{
    value: i64,
    base: Base,
}

impl Number 
{
    fn parse(input: &str) -> Result<Self, String> 
    {
        if input.is_empty() 
        {
            return Err("Empty number".into());
        }

        let base = Base::from_char(input.chars().next().unwrap())?; // The first character is the base
        let value_str = &input[1..]; // The rest is the value

        let value = match base 
        {
            Base::Decimal => value_str
                .parse()
                .map_err(|_| format!("Invalid decimal number: {}", value_str))?,
            Base::Hexadecimal => i64::from_str_radix(value_str, 16)
                .map_err(|_| format!("Invalid hexadecimal number: {}", value_str))?,
        };

        Ok(Number { value, base })
    }

    fn format(&self) -> String 
    {
        match self.base 
        {
            Base::Decimal => format!("d{}", self.value),
            Base::Hexadecimal => format!("h{:X}", self.value),
        }
    }
}

#[derive(Debug)]
enum Token 
{
    Number(Number),
    Operation(Operation),
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> 
{
    let mut tokens = Vec::new();
    let mut current_number = String::new();

    let chars: Vec<char> = expr.chars().filter(|c| !c.is_whitespace()).collect();
    let mut i = 0;

    while i < chars.len() 
    {
        match chars[i] {
            'd' | 'h' => 
            {
                if !current_number.is_empty() 
                {
                    tokens.push(Token::Number(Number::parse(&current_number)?));
                    current_number.clear();
                }
                current_number.push(chars[i]);
            }
            c if c.is_digit(16) => 
            {
                current_number.push(c);
            }
            c @ ('+' | '-' | '*' | '(' | ')') => 
            {
                if !current_number.is_empty() 
                {
                    tokens.push(Token::Number(Number::parse(&current_number)?));
                    current_number.clear();
                }
                tokens.push(Token::Operation(Operation::from_char(c)?));
            }
            _ => return Err(format!("Invalid character: {}", chars[i])),
        }
        i += 1;
    }

    if !current_number.is_empty() 
    {
        tokens.push(Token::Number(Number::parse(&current_number)?));
    }

    Ok(tokens)
}

// Converts infix tokens to Reverse Polish Notation using the shunting yard algorithm.
fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, String> 
{
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens 
    {
        match token 
        {
            Token::Number(_) => output.push(token),
            Token::Operation(Operation::OpenParen) => operator_stack.push(token),
            Token::Operation(Operation::CloseParen) => 
            {
                while let Some(Token::Operation(op)) = operator_stack.last() 
                {
                    if *op == Operation::OpenParen 
                    {
                        operator_stack.pop();
                        break;
                    }
                    output.push(operator_stack.pop().unwrap());
                }
            }
            Token::Operation(op) => 
            {
                while let Some(Token::Operation(top_op)) = operator_stack.last() 
                {
                    if *top_op == Operation::OpenParen || top_op.precedence() < op.precedence() 
                    {
                        break;
                    }
                    output.push(operator_stack.pop().unwrap());
                }
                operator_stack.push(Token::Operation(op));
            }
        }
    }

    while let Some(op) = operator_stack.pop() 
    {
        match op 
        {
            Token::Operation(Operation::OpenParen) => 
            {
                return Err("Unmatched open parenthesis".into());
            }
            _ => output.push(op),
        }
    }

    Ok(output)
}

// Evaluates the expression in Reverse Polish Notation.
fn evaluate_rpn(tokens: Vec<Token>) -> Result<i64, String> 
{
    let mut stack = Vec::new();

    for token in tokens 
    {
        match token 
        {
            Token::Number(num) => stack.push(num.value),
            Token::Operation(op) => 
            {
                let right = stack.pop().ok_or("Invalid expression")?;
                let left = stack.pop().ok_or("Invalid expression")?;
                stack.push(op.apply(left, right));
            }
        }
    }

    stack.pop().ok_or("Invalid expression".into())
}

fn process_expression(input: &str) -> Result<String, String> 
{
    let input = input.trim();
    if input.is_empty() 
    {
        return Err("Empty expression".into());
    }

    let output_base = Base::from_char(input.chars().last().ok_or("Empty expression")?)?;
    let expr = &input[..input.len() - 1].trim();

    let tokens = tokenize(expr)?;
    
    let rpn_tokens = shunting_yard(tokens)?;
    
    let result = evaluate_rpn(rpn_tokens)?;

    Ok(Number 
        {
            value: result,
            base: output_base,
        }.format()
    )

}

fn clear_console() {
    if cfg!(target_os = "windows") {
        // Windows
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        // Unix
        Command::new("clear").status().unwrap();
    }
}

fn main() -> rustyline::Result<()> 
{
    let mut rl = DefaultEditor::new()?;

    loop 
    {
        match rl.readline("Enter an expression (or 'q' to quit): ") 
        {
            Ok(line) => 
            {
                if line.is_empty() 
                {
                    continue;
                }
                if line.trim() == "q" 
                {
                    break;
                }
                if line.trim() == "c" 
                {
                    clear_console();
                    continue; 
                }
                rl.add_history_entry(&line)?;

                match process_expression(&line) 
                {
                    Ok(result) => println!("{}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => 
            {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
    Ok(())
}
