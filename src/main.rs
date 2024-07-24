mod command;

use clap::{Parser};

#[derive(Parser, Clone)]
struct App {
    path: std::path::PathBuf,
    cmd: String, // add, select, list, complete, delete, edit
    arg1: Option<String>
}

fn handle_cmd(args: App, context: String) {
    println!("{}", args.cmd);
    match args.cmd.as_str() {
        "add" => command::add(args, context),
        "select"|"sel" => command::select(args, context),
        "list"|"lst" => command::list(args, context),
        "complete"|"cmp" => command::complete(args, context),
        "delete"|"del" => command::delete(args, context),
        "edit"|"edt" => command::edit(args, context),
        _ => {}
    }
}

fn handle_path(args: App) {
    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(content) => { handle_cmd(args, content); }
        Err(_error) => {
            std::fs::File::create(&args.path).expect("File Error");
            handle_cmd(args, String::new());
        }
    }
}

fn main() {
    let args = App::parse();

    handle_path(args.clone());
}
