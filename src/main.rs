/*

Aleksandr S 
XOR CL Tool

*/

use std::io;
use clap::Parser;
use indicatif::ProgressBar;


#[derive(Parser)]
struct Cli {
    pattern: String,
}
fn main() {
    const N: u64 = 1 << 20;
    progress_bar(N, "Default progress bar ");

    println!("Enter 2 binary values to XOR. \nE.g: ???? 10101010 10100101");
    let pattern = std::env::args().nth(1).expect("Use 'Help' command to get list of commands.");

    let args = Cli {
        pattern: pattern,
    };
    
    if pattern == "Help" {
        println!("The two commands this application currently has is 'Help' and 'XOR\nThe usage for each is as follows:\nXOR input1 input2\nHelp");
    } else {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let inputs: Vec<&str> = input.trim().split_whitespace().collect();
        
                if inputs.len() == 2 {
                    let input1 = inputs[0];
                    let input2 = inputs[1];
    
                    let int1 = u64::from_str_radix(input1, 2).unwrap();
                    let int2 = u64::from_str_radix(input2, 2).unwrap();
    
                    let result  = int1^int2;
                    let b_result = format!("{:b}", result);
    
                    println!("{}", b_result);
                } else {
                    println!("Incorrect syntax.");
                }
    
            }
           
            Err(error) => println!("error: {}", error),
        }
    }
   
    
}

fn progress_bar(n: u64, _label: &str) {
    let pb = ProgressBar::new(n);

    let mut _sum = 0;
    for i in 0..n {
        // Any quick computation, followed by an update to the progress bar.
        _sum += 2 * i + 3;
        pb.inc(1);
    }
    pb.finish();

    println!("Finished");
}
