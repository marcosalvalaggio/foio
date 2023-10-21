mod command;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version)]
struct Cli {
    #[clap(short, long, value_parser)]
    op: String,
    #[clap(value_parser)]
    first: f32,
    #[clap(value_parser)]
    second: f32
}

fn main() {
    println!("Hello, seaman!");
    command::setup::write_foio_script(true);
    command::setup::change_permission_to_foio_script(true);
    command::open::execute_foio_script(String::from("page"));
    println!("CLI test");
    let cli = Cli::parse();
    let result = match cli.op.as_str() {
        "+" => cli.first + cli.second,
        "-" => cli.first - cli.second,
        _ => panic!("lezzo")
    };
    println!("{}", result);
}
