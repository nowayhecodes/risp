use rustyline::{Editor};

fn repl(e: &mut Lenv) -> Result<(), ()> {
    println!("Risp v0.1.0");
    println!("Use exit(), ctrl-c, or ctrl-d to exit this prompt.");

    let mut rl = Editor::<()>::new();
    if rl.load_history("./.risp-history.txt").is_err() {
        println!("No history found.");
    }

    loop {
        let input = rl.readline("risp>");
    }
}
