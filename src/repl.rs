use log::{info, warn};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn repl(e: &mut Lenv) -> Result<()> {
    println!("Risp v0.1.0");
    println!("Use exit(), ctrl-c, or ctrl-d to exit this prompt.");

    let mut rl = Editor::<()>::new();
    if rl.load_history("./.risp-history.txt").is_err() {
        println!("No history found.");
    }

    loop {
        let input = rl.readline("risp::0.1.0 > ");

        match input {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                print_eval_result(eval_str(e, &line));
            }
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                break;
            }
            Err(error) => {
                warn!("Error: {:?}", error);
                break;
            }
        }
    }

    rl.save_history("./.risp-history.txt")?;
    Ok(());
}

fn print_eval_result(result: RispResult) {
    match result {
        Ok(res) => print!("{}", res),
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn eval_str(e: &mut Lenv, s: &str) -> RispResult {
    let parsed = RispParser::parse(Rule::risp, s)?.next().unwrap();
    let mut lval_ptr = lval_read(parsed)?;
    lval_eval(e, &mut *lval_ptr)
}
