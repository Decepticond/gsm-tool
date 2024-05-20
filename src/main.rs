/*

Aleksandr S 
XOR CL Tool

*/
use std::io;
use indicatif::ProgressBar;


fn main() {
    // Progress bar
    const N: u64 = 1 << 20;
    // Progress bar

    println!("Enter 2 binary values to XOR. \nE.g: ???? 10101010 10100101");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let inputs: Vec<&str> = input.trim().split_whitespace().collect();
        
        // Ensures 2 inputs given, converts vals to integers for XOR operation, then back to binary.
            if inputs.len() == 2 {
                let input1 = inputs[0];
                let input2 = inputs[1];
    
                let int1 = u64::from_str_radix(input1, 2).unwrap();
                let int2 = u64::from_str_radix(input2, 2).unwrap();
        
                let result  = int1^int2;
                let b_result = format!("{:b}", result);
                println!("{}", b_result);
                progress_bar(N, "Default progress bar ");
            } else {
                println!("Incorrect syntax.");
            }
    
        }
            Err(error) => println!("error: {}", error),
    }
}
   
// (Quick) Progress bar
fn progress_bar(n: u64, _label: &str) {
    let pb = ProgressBar::new(n);

    let mut _sum = 0;
    for i in 0..n {
        _sum += 2 * i + 3;
        pb.inc(1);
    }
    pb.finish();

    println!("Operation complete");
}
