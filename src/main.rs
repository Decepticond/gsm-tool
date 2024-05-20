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
    let args = Cmd::parse();

    if args.subcmd.is_none() {
        println!("\n{WELCOME}\n");
    }

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

const WELCOME: &str = r" Welcome to the...
              .;.       ,;        '.       .;.              
              .ld,.     'xl.    .:c.     .'lc.            
               :00Okdc,..oOo;,,;cd:...,;cldd;               
               ,OX0xdxkk::xxl:;cdo;;llc::odo'               
               .kNKxcccl,'lx;..;xl',;;;,cdxl.               
                oXKkollc;.:xl::ld:.,:::coxx:                
                .o0kddxoc;;:oddo:;;coooldxc.                
                .;cdl;'';cc:;;;;:cc;'';loc;.                
                .ll,coc.   ,llll,.  'lxo:oo.                
                 :xl,;odc'.:dddd:.'lxxc:x0c                 
                 'odo;,cddodddddddxko:lk0k,                 
                 .ldddc,:odddddxxkxc:dOOOo.                 
                  ,ldddo;,lddxxxkd:ck0Okd;                  
                    .,:loc,cxkkkl:dkdc;..                   
                        .',.;oo:';;.                     
                
                      GSM cmdline tool. 
Run gsm-tool cmdlist for a list of subcommands and their descriptors.";