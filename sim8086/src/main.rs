use std::fs;
use std::env;

const REGISTER: [&'static str; 16] = [
    "al", "cl", "dl", "bl", 
    "ah", "ch", "dh", "bh", 
    "ax", "cx", "dx", "bx", 
    "sp", "bp", "si", "di" 
];

fn main() {
    let path = env::args().nth(1).unwrap();
    let bytes = fs::read(path).unwrap();
    let mut i = 0;
    while i < bytes.len() {
        let byte_1 = bytes[i+1];
        let byte_2 = bytes[i];

        let op_code = byte_1 >> 2;
        let direction = (byte_1 & 0b00000010) >> 1;
        let wide = byte_1 & 0b00000001;
        // println!("{:x}{:x}", byte_1, byte_2);
        // println!("op_code:\t{:b}", op_code);
        // println!("direction:\t{:b}", direction);
        // println!("wide:\t{:b}", wide);

        let mode = byte_2 >> 6;
        let reg_1 = (byte_2 & 0b00111000) >> 3;
        let reg_2 = byte_2 & 0b00000111;
        let reg_1_str = register_decoder(reg_1, wide);
        let reg_2_str = register_decoder(reg_2, wide);

        println!("{}, {}", reg_1_str, reg_2_str);
        i += 2;
    }
}

fn register_decoder(reg: u8, wide: u8) -> &'static str {
    let reg_index = ((wide << 3) | reg) as usize;
    return REGISTER[reg_index];
}