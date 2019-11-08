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

? fill current cell with random value between 0 and 255     V
~ fill current cell with random boolean                     V

A-Z temp variables

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

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    let mut f = 0;
    let mut g = 0;
    let mut h = 0;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut l = 0;
    let mut m = 0;
    let mut n = 0;
    let mut o = 0;
    let mut p = 0;
    let mut q = 0;
    let mut r = 0;
    let mut s = 0;
    let mut t = 0;
    let mut u = 0;
    let mut v = 0;
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

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
                println!(" ");
                for (i, v) in values.iter().enumerate(){
                    println!("value {}: {}", i, v)
                }
                println!(" ");
                println!("A is {}", a);
                println!("B is {}", b);
                println!("C is {}", c);
                println!("D is {}", d);
                println!("E is {}", e);
                println!("F is {}", f);
                println!("G is {}", g);
                println!("H is {}", h);
                println!("I is {}", i);
                println!("J is {}", j);
                println!("K is {}", k);
                println!("L is {}", l);
                println!("M is {}", m);
                println!("N is {}", n);
                println!("O is {}", o);
                println!("P is {}", p);
                println!("Q is {}", q);
                println!("R is {}", r);
                println!("S is {}", s);
                println!("T is {}", t);
                println!("U is {}", u);
                println!("V is {}", v);
                println!("W is {}", w);
                println!("X is {}", x);
                println!("Y is {}", y);
                println!("Z is {}", z);
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

            else if symbol == 'A'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = a;
                    a = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'B'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = b;
                    b = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'C'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = c;
                    c = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'D'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = d;
                    d = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'E'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = e;
                    e = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'F'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = f;
                    f = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'G'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = g;
                    g = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'H'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = h;
                    h = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'I'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = i;
                    i = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'J'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = j;
                    j = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'K'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = k;
                    k = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'L'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = l;
                    l = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'M'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = m;
                    m = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'N'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = n;
                    n = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'O'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = o;
                    o = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'P'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = p;
                    p = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'Q'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = q;
                    q = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'R'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = r;
                    r = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'S'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = s;
                    s = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'T'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = t;
                    t = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'U'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = u;
                    u = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'V'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = v;
                    v = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'W'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = w;
                    w = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'X'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = x;
                    x = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'Y'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = y;
                    y = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }

            else if symbol == 'Z'{
                if locked_cells.contains(&ptr) == false{
                    let tmp = z;
                    z = values[ptr as usize];
                    values[ptr as usize] = tmp;
                }else{
                    println!("[LF ERROR 102] CANT CHANGE VALUE OF CELL {} BECAUSE IT IS LOCKED WITH VALUE {}", ptr, values[ptr as usize]);
                }
            }
        }
        io::stdout().flush();
        println!(" ");
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