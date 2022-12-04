pub fn run_first(input: &str) -> String {
    version_sum(input).to_string()
}

pub fn run_second(input: &str) -> String {
    eval_packets(input).to_string()
}

#[derive(Eq, PartialEq, Debug)]
pub enum PacketType {
    LiteralValue = 0,
    Operator = 1,
}

#[derive(Eq, PartialEq, Debug)]
pub enum LengthType {
    SubPacketsTotalLength = 0,
    NumSubPackets = 1,
}

#[derive(Clone, Debug)]
pub struct Packet {
    version: u8,
    type_id: u8,
    subpackets: Vec<Packet>,
    value: usize,
}

fn parse_packet(mut offset: usize, binary: &str) -> (Packet, usize) {
    let version = version(offset, binary);
    offset += 3;
    let type_id = type_id(offset, binary);
    offset += 3;

    match packet_type(type_id) {
        PacketType::LiteralValue => parse_literal(offset, version, type_id, binary),
        PacketType::Operator => parse_operator(offset, version, type_id, binary),
    }
}

fn parse_literal(mut offset: usize, version: u8, type_id: u8, binary: &str) -> (Packet, usize) {
    let mut literal_str = "".to_string();

    while binary.chars().nth(offset).unwrap() == '1' {
        let start = offset + 1;
        let end = offset + 5;
        literal_str += &binary[start..end];
        offset += 5;
    }

    let start = offset + 1;
    let end = offset + 5;
    literal_str += &binary[start..end];

    offset += 5;
    let value = usize::from_str_radix(&literal_str, 2).unwrap();
    let packet = Packet {
        version,
        type_id,
        value,
        subpackets: vec![],
    };
    (packet, offset)
}

fn parse_operator(mut offset: usize, version: u8, type_id: u8, binary: &str) -> (Packet, usize) {
    let mut subpackets: Vec<Packet> = vec![];
    let len_type = length_type(offset, binary);
    offset += 1;

    match len_type {
        LengthType::SubPacketsTotalLength => {
            let len = subpackets_length(offset, binary);
            offset += 15;
            let target_offset = offset + len;
            while offset < target_offset {
                let (child_packet, new_offset) = parse_packet(offset, binary);
                subpackets.push(child_packet);
                offset = new_offset;
            }
        }
        LengthType::NumSubPackets => {
            let num_subpackets = subpackets_num(offset, binary);
            offset += 11;
            for _ in 0..num_subpackets {
                let (child_packet, new_offset) = parse_packet(offset, binary);
                subpackets.push(child_packet);
                offset = new_offset
            }
        }
    }

    (
        Packet {
            version,
            type_id,
            subpackets,
            value: 0,
        },
        offset,
    )
}

fn eval_packet(p: Packet) -> usize {
    let children: Vec<usize> = p.subpackets.into_iter().map(eval_packet).collect();
    match p.type_id {
        0 => children.into_iter().sum(),
        1 => children.into_iter().product(),
        2 => children.into_iter().min().unwrap(),
        3 => children.into_iter().max().unwrap(),
        4 => p.value,
        5 => (children[0] > children[1]).into(),
        6 => (children[0] < children[1]).into(),
        7 => (children[0] == children[1]).into(),
        _ => unreachable!(),
    }
}

fn sum_version(p: Packet) -> usize {
    p.version as usize + p.subpackets.into_iter().map(sum_version).sum::<usize>()
}

fn version_sum(input: &str) -> usize {
    let binary = hex_to_bin(input);
    let (p, _) = parse_packet(0, &binary);
    sum_version(p)
}

fn eval_packets(input: &str) -> usize {
    let binary = hex_to_bin(input);
    let (p, _) = parse_packet(0, &binary);
    eval_packet(p)
}

pub fn version(offset: usize, binary: &str) -> u8 {
    let version_str = &binary[offset..offset + 3];
    u8::from_str_radix(version_str, 2).unwrap()
}

pub fn subpackets_length(offset: usize, binary: &str) -> usize {
    let length_str = &binary[offset..offset + 15];
    usize::from_str_radix(length_str, 2).unwrap()
}

pub fn subpackets_num(offset: usize, binary: &str) -> usize {
    let num_str = &binary[offset..offset + 11];
    usize::from_str_radix(num_str, 2).unwrap()
}

fn hex_to_bin(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn type_id(offset: usize, binary: &str) -> u8 {
    let version_str = &binary[offset..offset + 3];
    u8::from_str_radix(version_str, 2).unwrap()
}

pub fn packet_type(type_id: u8) -> PacketType {
    match type_id {
        4 => PacketType::LiteralValue,
        _ => PacketType::Operator,
    }
}

fn length_type(offset: usize, binary: &str) -> LengthType {
    match binary.chars().nth(offset).unwrap() {
        '0' => LengthType::SubPacketsTotalLength,
        '1' => LengthType::NumSubPackets,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_to_string;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u8 = 16;

    #[test]
    fn test_part_1() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_first(&data);

        // then
        let expected = "925";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_part_2() {
        // given
        let data = read_to_string(YEAR, DAY, false);

        // when
        let result = run_second(&data);

        // then
        let expected = "342997120375";
        assert_eq!(result, expected.to_string());
    }

    #[test]
    fn test_version_sum_1() {
        let input = "D2FE28";
        let sum = version_sum(input);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_version_sum_2() {
        let input = "8A004A801A8002F478";
        let sum = version_sum(input);
        assert_eq!(sum, 16);
    }

    #[test]
    fn test_version_sum_3() {
        let input = "620080001611562C8802118E34";
        let sum = version_sum(input);
        assert_eq!(sum, 12);
    }

    #[test]
    fn test_version_sum_4() {
        let input = "C0015000016115A2E0802F182340";
        let sum = version_sum(input);
        assert_eq!(sum, 23);
    }

    #[test]
    fn test_version_sum_5() {
        let input = "A0016C880162017C3686B18A3D4780";
        let sum = version_sum(input);
        assert_eq!(sum, 31);
    }
}
