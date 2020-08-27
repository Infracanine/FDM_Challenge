fn main() {
    write_LCD(0123456789,1,1);
}


fn get_digits(num: i32) -> impl Iterator<Item = u32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
}

fn write_LCD(number: i32,width: i32,height:i32){
    let mut top = String::new();
    let mut middle = String::new();
    let mut bottom = String::new();
    
    for digit in get_digits(number){
        top.push(' ');
        match digit {
            0 => {
                top.push('_');

                middle.push('|');
                middle.push(' ');
                middle.push('|');

                bottom.push('|');
                bottom.push('_');
                bottom.push('|');
                
            }
            1 => {
                top.push(' ');
                
                middle.push_str("  |");

                bottom.push_str("  |");
            }
            2 => {
                top.push('_');

                middle.push(' ');
                middle.push('_');
                middle.push('|');

                bottom.push('|');
                bottom.push('_');
                bottom.push(' ');
            }
            3 => {
                top.push('_');

                middle.push(' ');
                middle.push('_');
                middle.push('|');

                bottom.push(' ');
                bottom.push('_');
                bottom.push('|');
            }
            4 => {
                top.push(' ');

                middle.push('|');
                middle.push('_');
                middle.push('|');

                bottom.push(' ');
                bottom.push(' ');
                bottom.push('|');
            }
            5 => {
                top.push('_');

                middle.push('|');
                middle.push('_');
                middle.push(' ');

                bottom.push(' ');
                bottom.push('_');
                bottom.push('|');
            }
            6 => {
                top.push('_');

                middle.push('|');
                middle.push('_');
                middle.push(' ');

                bottom.push('|');
                bottom.push('_');
                bottom.push('|');
            }
            7 => {
                top.push('_');

                middle.push(' ');
                middle.push(' ');
                middle.push('|');

                bottom.push(' ');
                bottom.push(' ');
                bottom.push('|');
            }
            8 => {
                top.push('_');

                middle.push('|');
                middle.push('_');
                middle.push('|');

                bottom.push('|');
                bottom.push('_');
                bottom.push('|');
            }
            9 => {
                top.push('_');

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
    println!("{}",top);
    println!("{}",middle);
    println!("{}",bottom);
}
