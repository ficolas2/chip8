use std::num::ParseIntError;

#[allow(unused)]

pub fn assemble(assembly_code: &str) -> Vec<u8> {
    let assembly_code = assembly_code.replace(";", "\n");
    let mut machine_code = Vec::new();

    for (i, line) in assembly_code.lines().enumerate() {
        let line = line.trim();
        let tokens: Vec<&str> = line.split(' ').filter(|&t| !t.is_empty()).collect();

        if tokens.is_empty() {
            continue;
        }

        let code = match tokens[0] {
            "cls" => 0x00E0,
            "rts" => 0x00EE,
            "jmp" => 0x1000 | parse_num_or_err(tokens[1], i),
            "jsr" => 0x2000 | parse_num_or_err(tokens[1], i),
            "skeq" => parse_xnn_or_xy(tokens, i, 0x3000, 0x5000),
            "skne" => parse_xnn_or_xy(tokens, i, 0x4000, 0x9000),
            "mov" => parse_xnn_or_xy(tokens, i, 0x6000, 0x8000),
            "add" => parse_xnn_or_xy(tokens, i, 0x7000, 0x8004),
            "or" => 0x8001 | parse_xy(tokens, i),
            "and" => 0x8002 | parse_xy(tokens, i),
            "xor" => 0x8003 | parse_xy(tokens, i),
            "sub" => 0x8005 | parse_xy(tokens, i),
            "shr" => 0x8006 | parse_reg_or_err(tokens[1], i),
            "rsb" => 0x8007 | parse_xy(tokens, i),
            "shl" => 0x800E | parse_reg_or_err(tokens[1], i),
            "mvi" => 0xA000 | parse_num_or_err(tokens[1], i),
            "jmi" => 0xB000 | parse_num_or_err(tokens[1], i),
            "rand" => 0xC000 | parse_xnn(tokens, i),
            "sprite" => 0xD000 | parse_xyn(tokens, i),
            "skpr" => 0xE09E | parse_reg_or_err(tokens[1], i) << 8,
            "skup" => 0xE0A1 | parse_reg_or_err(tokens[1], i) << 8,
            "gdelay" => 0xF007 | parse_reg_or_err(tokens[1], i) << 8,
            "key" => 0xF00A | parse_reg_or_err(tokens[1], i) << 8,
            "sdelay" => 0xF015 | parse_reg_or_err(tokens[1], i) << 8,
            "ssound" => 0xF018 | parse_reg_or_err(tokens[1], i) << 8,
            "adi" => 0xF01E | parse_reg_or_err(tokens[1], i) << 8,
            "font" => 0xF029 | parse_reg_or_err(tokens[1], i) << 8,
            "bcd" => 0xF033 | parse_reg_or_err(tokens[1], i) << 8,
            "str" => 0xF055 | parse_reg_or_err(tokens[1], i) << 8,
            "ldr" => 0xF065 | parse_reg_or_err(tokens[1], i) << 8,

            "end" => 0x0000,
            _ => {
                panic!("Unknown instruction: {}", tokens[0]);
            }
        };

        machine_code.push(((code & 0xFF00) >> 8) as u8);
        machine_code.push((code & 0x00FF) as u8);
    }

    machine_code
}

fn parse_xyn(tokens: Vec<&str>, line: usize) -> u16 {
    let reg1 = parse_reg_or_err(tokens[1], line);
    let reg2 = parse_reg_or_err(tokens[2], line);
    let n = parse_num_or_err(tokens[3], line);
    (reg1 << 8) | (reg2 << 4) | n
}

fn parse_xnn(tokens: Vec<&str>, line: usize) -> u16 {
    let reg = parse_reg_or_err(tokens[1], line);
    (reg << 8) | parse_num_or_err(tokens[2], line)
}

// OP vX vY -> 0x_XY_
fn parse_xy(tokens: Vec<&str>, line: usize) -> u16 {
    let reg1 = parse_reg_or_err(tokens[1], line);
    let reg2 = parse_reg_or_err(tokens[2], line);
    (reg1 << 8) | (reg2 << 4)
}

fn parse_xnn_or_xy(tokens: Vec<&str>, line: usize, base_xnn: u16, base_xy: u16) -> u16 {
    let reg = parse_reg_or_err(tokens[1], line);
    if tokens[2].starts_with("v") {
        let reg2 = parse_reg_or_err(tokens[2], line);
        base_xy | (reg << 8) | (reg2 << 4)
    } else {
        base_xnn | (reg << 8) | parse_num_or_err(tokens[2], line)
    }
}

fn parse_reg_or_err(token: &str, line: usize) -> u16 {
    let num = token
        .strip_prefix("v")
        .unwrap_or_else(|| panic!("Expected a register at line {}", line));
    parse_u16(num).unwrap_or_else(|_| panic!("Invalid register at line {}", line))
}

fn parse_num_or_err(token: &str, line: usize) -> u16 {
    parse_u16(token).unwrap_or_else(|_| panic!("Invalid number at line {}", line))
}

fn parse_u16(token: &str) -> Result<u16, ParseIntError> {
    token
        .strip_prefix("0x")
        .map_or_else(|| token.parse::<u16>(), |hex| u16::from_str_radix(hex, 16))
}
