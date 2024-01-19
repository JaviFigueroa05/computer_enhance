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
        let byte_1 = bytes[i];
        let byte_2 = bytes[i+1];

        let _op_code = byte_1 >> 2;
        let _direction = (byte_1 & 0b00000010) >> 1;
        let wide = byte_1 & 0b00000001;

        let _mode = byte_2 >> 6;
        let reg = (byte_2 & 0b00111000) >> 3;
        let reg_mem = byte_2 & 0b00000111;
        let reg_str = register_decoder(reg, wide);
        let reg_mem_str = register_decoder(reg_mem, wide);

        println!("mov {}, {}", reg_mem_str, reg_str);
        i += 2;
    }
}

fn register_decoder(reg: u8, wide: u8) -> &'static str {
    let reg_index = ((wide << 3) | reg) as usize;
    return REGISTER[reg_index];
}