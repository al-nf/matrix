use rustyline::DefaultEditor;
use crate::eval::Evaluator;

pub fn start() {
    let mut rl = DefaultEditor::new().expect("");
    let mut eval = Evaluator::new();

    loop {
        match rl.readline(">>> ") {
            Ok(line) => {
                let input = line.trim();
                if input == "exit" { break; }
                if input.is_empty() { continue; }

                match eval.eval_line(input) {
                    Ok(Some(result)) => println!("{}", result),
                    Ok(None) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(_) => break,
        }
    }
}

