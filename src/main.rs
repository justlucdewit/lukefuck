//started on 6-11-2019

use std::*;
use std::io::Write;
use std::env;
use std::convert::TryInto;

use rand::Rng;
/*
+ increase current value    V M
- decrease current value    V M
> increase pointer pos      V
< decrease pointer pos      V
[ loop open                 V
] loop close                V
. output current value      V
, input into current value  V

! debug                     V
@ reset pointer             V
# reset everything          V EM
; reset current value       V M

/ mark current cell as locked   V
| mark current cell as open     V

? fill current cell with random value between 0 and 255     X
~ fill current cell with random boolean                     X

*/

//-------------------------------------------helper functions---------------------

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

//------------------------------------main program---------------------------------

fn interpreter(filecode:&str){
    let mut values = vec![0 as i128];
    let mut ptr : i128 = 0;
    let mut loop_mem = vec![];
    let mut locked_cells = vec![];
    loop{
        let mut code = filecode.to_string();
        if filecode == "pg"{
            code = input();
        }

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
                if locked_cells.contains(&ptr) == false{
                    if values[ptr as usize] == 255{
                        values[ptr as usize] = 0;
                    }else{
                        values[ptr as usize] += 1;
                    }
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == '-'{
                if locked_cells.contains(&ptr) == false{
                    if values[ptr as usize] == 0{
                        values[ptr as usize] = 255;
                    }else{
                        values[ptr as usize] -= 1;
                    }
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
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

            else if symbol == '@'{
                ptr = 0;
            }

            else if symbol == '#'{
                if locked_cells.len() == 0{
                    ptr = 0;
                    values = vec![0];
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == ';'{
                if locked_cells.contains(&ptr) == false{
                    values[ptr as usize] = 0;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == '/'{
                if locked_cells.contains(&ptr){
                    println!("[ERROR 103] CANT LOCK CELL BECAUSE A CELL IS STILL LOCKED");
                }else{
                    locked_cells.push(ptr);
                }
            }

            else if symbol == '|'{
                if locked_cells.contains(&ptr){
                    //delete value from vector
                    let mut index_of_locked = 69420;
                    for (i, cell) in locked_cells.iter().enumerate(){
                        if cell == &ptr{
                            index_of_locked = i;
                            break;
                        }
                    }
                    println!("removing index {}", index_of_locked);
                    locked_cells.remove(index_of_locked);
                }else{
                    println!("[ERROR 103] CANT UNLOCK CELL BECAUSE CELL {} IS ALREADY UNLOCKED", ptr);
                }
            }

            else if symbol == '?'{
                if locked_cells.contains(&ptr) == false{
                    let r : u8 = rand::thread_rng().gen_range(0, 255);
                    values[ptr as usize] = r.try_into().unwrap();
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == '~'{
                if locked_cells.contains(&ptr) == false{
                    let r : u8 = rand::thread_rng().gen_range(0, 2);
                    values[ptr as usize] = r.try_into().unwrap();
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }
        }
        io::stdout().flush();
        println!("\n");

        if filecode != "pg"{
            break;
        }
    }
}


fn main(){
    let args = get_args();
    const VERSION : &str = "Lukefuck 0.0.2a";
    println!("{}", VERSION);

    if args[1] == "playground"{
        interpreter("pg");
    }

    else if args[1]== "run"{
        let path = &args[2];
        let code = read_file(path);
        interpreter(&code);
    }
    
    else{
        println!("command not understood, try 'Lukefuck playground'");
    }
}

pub fn read_file(filename: &str) -> String {
    return fs::read_to_string(filename).expect("ERROR WHILE LOADING CANT FIND FILE");
}