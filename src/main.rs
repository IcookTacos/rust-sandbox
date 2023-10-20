mod rustbyexample;
use std::env;
use std::process;
use colored::Colorize;

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
 * and this help message adjusts the printout for what tasks are available
 */
fn help_message(){
    println!("Chose what task you want to run (1-2) \nexample: cargo run --task1");
    process::exit(1);
}

fn run_task(task: String){
    println!("Running task");
    if task == "task1" { task1() }
    if task == "task2" { task2() }
    unknown_task();
}

fn task1(){
    rustbyexample::hello_world();
    rustbyexample::formatted_print();
    rustbyexample::debug_print();
    complete_task();
}

fn task2(){
    rustbyexample::primitive_scalars();
    complete_task();
}

fn complete_task(){
    println!("{}","All tasks complete!".green());
    process::exit(1);
}

fn unknown_task(){
    println!("{} Unknown task","Error:".red());
    process::exit(1);
}
