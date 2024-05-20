/*

Aleksandr S 
GSM Tools for nonspecific usecases

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

    //#[command(about = "unknown ")] -- don't uncomment until I figure out what to make next
    //Untool, 

    //#[command(about = "unknown ")]
    //Untool2,
}

fn main() {
    println!("    ...           ..       ...                    ...              .. 
    ..              .     ...    ......         ...                .. 
   ..                 .    ..          ...     ..                  .. 
   ..                  ..   ..            ... ..                   .. 
   ..                   ... ...             ....                   .. 
   .                     ......               ..                   .. 
   .                   ..........                                  .. 
                      ............                                 .. 
   .                                                               .. 
   ..                                                              .. 
   ..                                                              .. 
    ..                                                            ..  
     ..                                  .....''........         ..   
      ..     ........''''''.             .,,,,,,'.   ..         ..    
      ...    ...    .',,,,,..            .,,,,,,..    ..      ...     
..........   ..     .',,,,'.            ..,,,,,,.     ..      ..  ..  
... ....     ..     .',,,,'.             .',,,,'.      ..         ..  
  ..         ..      .',,,'.              .',''.       .         .    
   ...       ..       .....    ..           ..                ..      
     ...                                           .....     ...      
      .    ...                                      ..         ..     
     .                         ...........                      ..    
    ..                     .....                                 ..   
   ...........                                        .......         
              ......                                                  
                 .......                        ..                    
                   ..                            .                    
                     ..                                               
                      ..                                              
                    ...                                               
                    .....                            .                
                       ..                            .                
                      ..                              .               
                      ..                               .              
                      .                                .    \nWelcome to a GSM cmdline tool. Run gsm-tool cmdlist for a list of all subcommands and their descriptors.");
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
