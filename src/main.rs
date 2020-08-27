use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{},{},{}",args[1],args[2],args[3]);
    let number : isize = args[1].parse().unwrap();
    let width : isize = args[2].parse().unwrap();
    let height : isize = args[3].parse().unwrap();
    write_LCD(number,width,height);
}


fn get_digits(num: isize) -> impl Iterator<Item = u32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
}

fn write_LCD(number: isize,width: isize,height:isize){
    let mut top = String::new();
    let mut middle = String::new();
    let mut bottom = String::new();

    if (width == 0 || height == 0){
        panic!("invalid value of height or width");
    }

    for digit in get_digits(number){
        top.push(' ');

        match digit {
            0 => {
                for i in 0..width{
                    top.push('_');
                }

                middle.push('|');
                for i in 0..width{
                    middle.push(' ');
                }

                middle.push('|');

                bottom.push('|');
                bottom.push('_');
                bottom.push('|');
                
            }
            1 => {
                for i in 0..width{
                    top.push(' ');
                }
                
                middle.push(' ');
                middle.push('|');
                middle.push(' ');

                bottom.push(' ');
                bottom.push('|');
                bottom.push(' ');
            }
            2 => {
                for i in 0..width{
                    top.push('_');
                }

                middle.push(' ');
                middle.push('_');
                middle.push('|');

                bottom.push('|');
                bottom.push('_');
                bottom.push(' ');
            }
            3 => {
                for i in 0..width{
                    top.push('_');
                }


                middle.push(' ');
                middle.push('_');
                middle.push('|');

                bottom.push(' ');
                bottom.push('_');
                bottom.push('|');
            }
            4 => {
                for i in 0..width{
                    top.push(' ');
                }


                middle.push('|');
                middle.push('_');
                middle.push('|');

                bottom.push(' ');
                bottom.push(' ');
                bottom.push('|');
            }
            5 => {
                for i in 0..width{
                    top.push('_');
                }

                middle.push('|');
                middle.push('_');
                middle.push(' ');

                bottom.push(' ');
                bottom.push('_');
                bottom.push('|');
            }
            6 => {
                for i in 0..width{
                    top.push('_');
                }


                middle.push('|');
                middle.push('_');
                middle.push(' ');

                bottom.push('|');
                bottom.push('_');
                bottom.push('|');
            }
            7 => {
                for i in 0..width{
                    top.push('_');
                }


                middle.push(' ');
                middle.push(' ');
                middle.push('|');

                bottom.push(' ');
                bottom.push(' ');
                bottom.push('|');
            }
            8 => {
                for i in 0..width{
                    top.push('_');
                }


                middle.push('|');
                middle.push('_');
                middle.push('|');

                bottom.push('|');
                bottom.push('_');
                bottom.push('|');
            }
            9 => {
                for i in 0..width{
                    top.push('_');
                }
                

                middle.push('|');
                middle.push('_');
                middle.push('|');

                bottom.push(' ');
                bottom.push('_');
                bottom.push('|');
            }
            _ => eprintln!("Invalid number! Expecting 0-9")
        }

        top.push(' ');
    }

    // Print final contents of top middle and bottom to console
    println!("{}",top);
    println!("{}",middle);
    println!("{}",bottom);
}
