use crc32fast::Hasher;
use chrono::Utc;

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

// call generate with required info to create unique name
fn get_id(ip_address: String, index: u16) -> String {
    let dt = Utc::now();

    let mut hasher = Hasher::new();
    hasher.update(ip_address.as_bytes());
    let ip_checksum = hasher.finalize();

    let mut num: u128 = format!("{}{}{}", index, dt.timestamp_micros(), ip_checksum)
        .parse()
        .unwrap();

    generate(&mut num)
}

// test with random data
fn main() {
    for i in 90..110 {
        let result = get_id("192.168.001.231".to_string(), i);
        println!("{}", result);
    }
}

