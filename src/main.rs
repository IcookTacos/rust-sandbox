mod rustbyexample;
use std::env;
use std::process;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("cargo run <task> \ncargo run help");
        process::exit(1);
    }
    let arg = &args[1];
    if arg == "help" { help_message(); }
    run_task(arg.to_string());
}

/*
 * It would be nice to make this dynamical, say I somehow keep track of what taks I've completed
 * and this help message adjustds the printout for what tasks are available
 */
fn help_message(){
    println!("Chose what task you want to run (1-2) \nexample: cargo run --task1");
    process::exit(1);
}

fn run_task(task: String){
    println!("Running task");
    if task == "task1" { task1() }
    println!("Unknown task");
    process::exit(1);
}

fn task1(){
    rustbyexample::hello_world();
    rustbyexample::formatted_print();
    rustbyexample::debug_print();
}
