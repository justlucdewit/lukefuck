//started on 6-11-2019

use std::*;
use std::io::Write;
use std::env;
use std::convert::TryInto;
/*
+ increase current value    x
- decrease current value    x
> increase pointer pos      V
< decrease pointer pos      V
[ loop open                 x
] loop close                x
. output current value      x
, input into current value  x

! debug                     V
*/

fn input() -> String {
    let mut code = String::new();

    io::stdin().read_line(&mut code);
    return code;
}

pub fn get_args() -> Vec<String> {
    return env::args().collect();
}

fn output(byte : u8){
    print!("{}", byte as char);
}

fn interpreter(){
    let mut values = vec![0];
    let mut ptr : i128 = 0;
    loop{
        let code = input();
        for symbol in code.chars(){
            if symbol == '>'{
                ptr+=1;
                if values.len()-1 < ptr as usize{
                    values.push(0);
                }
            }

            else if symbol == '<'{
                ptr-=1;
            }

            else if symbol == '+'{
                values[ptr as usize] += 1;
            }

            else if symbol == '-'{
                values[ptr as usize] -= 1;
            }

            else if symbol == '.'{
                output(values[ptr as usize]);
            }

            else if symbol == ','{
                let tmp = input();
                let mut str = tmp.trim();
                if str.len() == 1{
                    let byte = str.chars().next().unwrap() as u8;
                    values[ptr as usize] = byte;
                }else{
                    println!("[LF ERROR 001] CANT INPUT MORE THEN 1 CHARACTER");
                }
            }

            else if symbol == '!'{
                println!("pointer position: {}", ptr);
                for (i, v) in values.iter().enumerate(){
                    println!("value {}: {}", i, v)
                }
            }
        }
        io::stdout().flush();
        println!("\n");
    }
}

fn main(){
    let args = get_args();
    const VERSION : &str = "Lukefuck 0.0.0b";
    println!("{}", VERSION);

    if args[1] == "playground"{
        interpreter();
    }
}