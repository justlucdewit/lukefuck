use std::io;
use std::env;
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

fn input() -> std::string::String {
    let mut code = String::new();

    io::stdin().read_line(&mut code);
    return code;
}

pub fn get_args() -> Vec<String> {
    return env::args().collect();
}

fn interpreter(){
    let values = vec![0];
    let mut ptr = 0;
    loop{
        let code = input();
        for symbol in code.chars(){
            if symbol == '>'{
                ptr+=1;
                if true/*test if ptr index is in values*/{

                }
            }

            else if symbol == '<'{
                ptr-=1;
                if ptr < 0{
                    ptr = 0;
                }
            }

            else if symbol == '!'{
                println!("pointer position: {}", ptr);
                let v = "";
                for (i, v) in values.iter().enumerate(){
                    println!("value {}: {}", i, v)
                }
                print!("\n");
            }
        }
    }
}

fn main(){
    let args = get_args();
    const VERSION : &str = "Lukefuck 0.0.0a";
    println!("{}", VERSION);

    if args[1] == "playground"{
        interpreter();
    }
}
