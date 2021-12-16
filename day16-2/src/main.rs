use std::io::BufRead;

#[derive(Debug)]
enum Operation {
    SUM,
    PRODUCT,
    MIN,
    MAX,
    GT,
    LT,
    EQ,
}

#[derive(Debug)]
enum Packet {
    LiteralValue(u8, u64),
    Operator(u8, Operation, Vec<Packet>),
}

impl Packet {
    fn value(&self) -> u64 {
        match self {
            Packet::LiteralValue(_, v) => *v,
            Packet::Operator(_, op, subpackets) => {
                match op {
                    Operation::SUM => subpackets.iter().map(|x| x.value()).sum(),
                    Operation::PRODUCT => subpackets.iter().map(|x| x.value()).product(),
                    Operation::MIN => subpackets.iter().map(|x| x.value()).min().unwrap(),
                    Operation::MAX => subpackets.iter().map(|x| x.value()).max().unwrap(),
                    Operation::GT => {
                        if subpackets[0].value() > subpackets[1].value() {
                            1
                        } else {
                            0
                        }
                    },
                    Operation::LT => {
                        if subpackets[0].value() < subpackets[1].value() {
                            1
                        } else {
                            0
                        }
                    },
                    Operation::EQ => {
                        if subpackets[0].value() == subpackets[1].value() {
                            1
                        } else {
                            0
                        }
                    },
                }
            }
        }
    }
}

struct PacketData {
    data: Vec<u8>,
    idx: usize,
}

impl PacketData {
    fn read_bits(&mut self, count: usize) -> u32 {
        assert!(count < 16);
        let mut result: u32 = 0;

        for idx in 0..count {
            let byte_idx = self.idx / 8;
            let bit_idx = 7 - (self.idx % 8);
            let byte = self.data[byte_idx];
            let bit = (byte & (1 << bit_idx)) as u32 >> bit_idx << (count - idx - 1);

            // println!("IDX {} (BIT IDX {})\n\n{:08b}\n{:08b} (1 << bit_idx)\n{:08b} (bit)\n{:08b} (result)\n\n\n", self.idx, bit_idx, byte, 1 << bit_idx, bit, result);

            result |= bit as u32;
            self.idx += 1;
        }

        result
    }
}

impl Packet {
    fn parse(data: &mut PacketData) -> Packet {
        let version = data.read_bits(3) as u8;
        let typ = data.read_bits(3);

        match typ {
            4 => {
                let mut has_more = true;
                let mut groups = vec![];

                while has_more {
                    let group = data.read_bits(5);
                    has_more = group & 0b10000 > 0;
                    groups.push(group & 0b1111);
                }

                let mut value = 0u64;
                for (idx, group) in groups.iter().rev().enumerate() {
                    value |= (*group as u64) << idx * 4;
                }

                Packet::LiteralValue(version, value)
            },
            typ => {
                let length_type = data.read_bits(1);

                let mut subpackets = vec![];

                if length_type == 0 {
                    let len = data.read_bits(15) as usize;
                    let mark = data.idx;
                    while data.idx < mark + len {
                        subpackets.push(Packet::parse(data));
                    }
                }
                else {
                    let packet_count = data.read_bits(11);
                    for i in 0..packet_count {
                        subpackets.push(Packet::parse(data));
                    }
                }

                Packet::Operator(version, match typ {
                    0 => Operation::SUM,
                    1 => Operation::PRODUCT,
                    2 => Operation::MIN,
                    3 => Operation::MAX,
                    5 => Operation::GT,
                    6 => Operation::LT,
                    7 => Operation::EQ,
                    _ => unreachable!(),
                }, subpackets)
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut raw = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        while let Some(upper) = chars.next() {
            let lower = chars.next().unwrap_or('0');
            raw.push(((upper.to_digit(16).unwrap() << 4) | lower.to_digit(16).unwrap()) as u8);
        }
    }

    let mut data = PacketData{data: raw, idx: 0};

    let packet = Packet::parse(&mut data);
    println!("{:?}", packet.value());
}
