/*

Aleksandr S 
XOR CL Tool

*/

use clap::Parser;
mod xor_tool;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cmd {
    #[command(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(about = "XOR sub-command. Takes 2 binary values, XORs them, and outputs the result.")]
    Xortool,

    #[command(about = "Outputs a list of commands ")]
    Cmdlist,
}

fn main() {
    let args = Cmd::parse();

match args.subcmd {
    Some(subcmd) => match subcmd {
        SubCommand::Xortool => xor_tool::xor_tool(),
        SubCommand::Cmdlist => println!("Test output"),
    },
    None => {
        println!("No targets given."); 
    }
}
}
