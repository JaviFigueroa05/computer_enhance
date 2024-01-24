use std::fs;
use std::env;
use bitmatch::bitmatch;

fn main() {
    let path = env::args().nth(1).unwrap();
    let bytes = fs::read(path).unwrap();
    let mut i = 0;
    while i < bytes.len() {
        i = instruction_decoder(&bytes, i);
    }
}

#[bitmatch]
fn mov_reg_mem_to_from_reg(byte_1: u8, byte_2: u8) -> String {
    #[bitmatch]
    let "???????w" = byte_1;
    #[bitmatch]
    let "??rrrmmm" = byte_2;

    let reg_str = register_decoder(r, w);
    let reg_mem_str = register_decoder(m, w);

    return format!("mov {}, {}", reg_mem_str, reg_str);
}

fn instruction_decoder(bytes: &Vec<u8>, current_byte_index: usize) -> usize {
        let byte_1 = bytes[current_byte_index];
        let byte_2 = bytes[current_byte_index+1];
        let instruction = mov_reg_mem_to_from_reg(byte_1, byte_2);
        println!("{}", instruction);

        return current_byte_index + 2;
}

fn register_decoder(reg: u8, wide: u8) -> &'static str {
    const REGISTER: [&'static str; 16] = [
        "al", "cl", "dl", "bl", 
        "ah", "ch", "dh", "bh", 
        "ax", "cx", "dx", "bx", 
        "sp", "bp", "si", "di" 
    ];
    let reg_index = ((wide << 3) | reg) as usize;
    return REGISTER[reg_index];
}