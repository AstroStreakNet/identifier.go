
// convert decimal numerals to base-62
fn generate(num: &mut u128) -> String {
    let mut unique_name: String = Default::default();
    let mut temp_remainder: u8;
    let mut temp_numeral: u8;
    let mut temp_char: char;

    while *num > 0 {
        temp_remainder = (*num % 62) as u8;
        temp_numeral = temp_remainder + if temp_remainder > 35 {
            61
        } else if temp_remainder > 9 {
            55
        } else {
            48
        };

        temp_char = temp_numeral as char;
        unique_name.push(temp_char);
        *num /= 62;
    }

    unique_name
}

fn testing() {
    let mut num: u128;
    let mut result: String;

    num = u128::MAX;
    result = generate(&mut num);
    println!("filename: {result}");

    num = 2552552552553112999923595959999;
    result = generate(&mut num);
    println!("filename: {result}");
}

fn main() {
    testing();
}

