/*

Aleksandr S 
XOR CL Tool

*/

use std::io;

fn main() {
    println!("Enter two binary values to XOR, seperate each by a space, ex: ???? 10101010 10100101");

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