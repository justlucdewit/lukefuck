//started on 6-11-2019

use std::*;
use std::io::Write;
use std::env;
use std::convert::TryInto;
/*
+ increase current value    V
- decrease current value    V
> increase pointer pos      V
< decrease pointer pos      V
[ loop open                 x
] loop close                x
. output current value      V
, input into current value  V

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
    let mut values = vec![0 as i128];
    let mut ptr : i128 = 0;
    let mut loop_mem = vec![];
    loop{
        let code = input();
        let mut reading_index : i128 = -1;
        loop{
            reading_index += 1;
            if reading_index >= code.len() as i128{
                break;
            }

            let symbol = code.chars().nth(reading_index as usize).unwrap();

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
                if values[ptr as usize] == 255{
                    values[ptr as usize] = 0;
                }else{
                    values[ptr as usize] += 1;
                }
            }

            else if symbol == '-'{
                if values[ptr as usize] == 0{
                    values[ptr as usize] = 255;
                }else{
                    values[ptr as usize] -= 1;
                }
            }

            else if symbol == '.'{
                output(values[ptr as usize].try_into().unwrap());
            }

            else if symbol == ','{
                let tmp = input();
                let mut str = tmp.trim();
                if str.len() == 1{
                    let byte = str.chars().next().unwrap() as u8;
                    values[ptr as usize] = byte.try_into().unwrap();
                }else{
                    println!("[LF ERROR 001] CANT INPUT MORE THEN 1 CHARACTER");
                }
            }

            else if symbol == '['{
                loop_mem.push(reading_index);
            }

            else if symbol == ']'{
                if values[ptr as usize] == 0{
                    loop_mem.pop();
                }else{
                    reading_index = loop_mem[loop_mem.len()-1];
                }
            }

            else if symbol == '!'{
                println!("pointer position: {}", ptr);
                println!("\n");
                for (i, v) in values.iter().enumerate(){
                    println!("value {}: {}", i, v)
                }
                println!("\n");
                for (i, v) in loop_mem.iter().enumerate(){
                    println!("loop mem {}: {}", i, v)
                }
            }
        }
        io::stdout().flush();
        println!("\n");
    }
}

fn main(){
    let args = get_args();
    const VERSION : &str = "Lukefuck 0.0.1b";
    println!("{}", VERSION);

    if args[1] == "playground"{
        interpreter();
    }
}