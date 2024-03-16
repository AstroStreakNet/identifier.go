use blake2::{Blake2b, Digest};

// convert decimal numerals to base-62
fn generate(num: &mut u64) -> String {
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
    let mut num: u64;
    let result: String;

    let ip_address = "192.168.213.213";

    let mut hasher = Blake2b::new();
    hasher.update(ip_address.as_bytes());
    let hash_result = hasher.finalize();

    let truncated_hash_hex = hash_result[..4]
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    num = 3112999923595959999;
    result = generate(&mut num);
    println!("filename: {truncated_hash_hex}{result}");
}

fn main() {
    testing();
}


