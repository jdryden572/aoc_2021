
const INPUT: &str = "E054831006016008CF01CED7CDB2D495A473336CF7B8C8318021C00FACFD3125B9FA624BD3DBB7968C0179DFDBD196FAE5400974A974B55C24DC580085925D5007E2D49C6579E49252E28600B580272379054AF57A54D65E1586A951D860400434E36080410926624D25458890A006CA251006573D2DFCBF4016919CC0A467302100565CF24B7A9C36B0402840002150CA3E46000042621C108F0200CC5C8551EA47F79FC28401C20042E0EC288D4600F42585F1F88010C8C709235180272B3DCAD95DC005F6671379988A1380372D8FF1127BDC0D834600BC9334EA5880333E7F3C6B2FBE1B98025600A8803F04E2E45700043E34C5F8A72DDC6B7E8E400C01797D02D002052637263CE016CE5E5C8CC9E4B369E7051304F3509627A907C97BCF66008500521395A62553A9CAD312A9CCCEAF63A500A2631CCD8065681D2479371E4A90E024AD69AAEBE20002A84ACA51EE0365B74A6BF4B2CC178153399F3BACC68CF3F50840095A33CBD7EF1393459E2C3004340109596AB6DEBF9A95CACB55B6F5FCD4A24580400A8586009C70C00D44401D8AB11A210002190DE1BC43872C006C45299463005EC0169AFFF6F9273269B89F4F80100507C00A84EB34B5F2772CB122D26016CA88C9BCC8BD4A05CA2CCABF90030534D3226B32D040147F802537B888CD59265C3CC01498A6B7BA7A1A08F005C401C86B10A358803D1FE24419300524F32AD2C6DA009080330DE2941B1006618450822A009C68998C1E0C017C0041A450A554A582D8034797FD73D4396C1848FC0A6F14503004340169D96BE1B11674A4804CD9DC26D006E20008747585D0AC001088550560F9019B0E004080160058798012804E4801232C0437B00F70A005100CFEE007A8010C02553007FC801A5100530C00F4B0027EE004CA64A480287C005E27EEE13DD83447D3009E754E29CDB5CD3C";

fn main() {
    println!("Answer one: {}", part1(INPUT));
}

fn part1(input: &str) -> usize {
    let mut binary = Binary::from_hex(input);
    let packet = parse_packet(&mut binary);

    sum_versions(&packet)
}

fn sum_versions(packet: &Packet) -> usize {
    match &packet.payload {
        Payload::Literal(_) => packet.version,
        Payload::Operator(packets) => packet.version + packets.iter().map(sum_versions).sum::<usize>(),
    }
}

fn parse_packet(binary: &mut Binary) -> Packet {
    let version = parse_binary_num(binary.take(3));
    let type_id = parse_binary_num(binary.take(3));
    let payload = match type_id {
        4 => {
            println!("Literal (version {})", version);
            get_literal_payload(binary)
        },
        _ => {
            println!("Operator (version {})", version);
            get_operator_payload(binary)
        },
    };
    Packet { version, payload }
}

fn get_literal_payload(binary: &mut Binary) -> Payload {
    let mut bits = String::new();
    loop {
        let group = binary.take(5);
        let label = &group[0..1];
        bits.push_str(&group[1..5]);
        if label == "0" { 
            break; 
        }
    }
    let num = parse_binary_num(&bits);
    Payload::Literal(num)
}

fn get_operator_payload(binary: &mut Binary) -> Payload {
    let mut packets = Vec::new();
    let length_type = binary.take(1);
    if length_type == "0" {
        // length is in bits
        let length = parse_binary_num(binary.take(15));
        println!("{} bits of sub-packets", length);
        let current_pos = binary.position;
        while binary.position < current_pos + length {
            packets.push(parse_packet(binary));
        }
    } else {
        let length = parse_binary_num(binary.take(11));
        println!("{} sub-packets", length);
        for _ in 0..length {
            packets.push(parse_packet(binary));
        }
    };

    Payload::Operator(packets)
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: usize,
    payload: Payload,
}

#[derive(Debug, PartialEq, Eq)]
enum Payload {
    Literal(usize),
    Operator(Vec<Packet>),
}

struct Binary {
    data: String,
    position: usize,
}

impl Binary {
    fn from_hex(hex: &str) -> Self {
        let bytes = hex::decode(hex).unwrap();
        let mut data = String::new();
        for byte in bytes {
            data.push_str(&format!("{:08b}", byte));
        }
        Self { data, position: 0 }
    }

    fn take(&mut self, n: usize) -> &str {
        let start = self.position;
        self.position += n;
        &self.data[start..start + n]
    }
}

fn parse_binary_num(input: &str) -> usize {
    usize::from_str_radix(input, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(16, part1("8A004A801A8002F478"));
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(12, part1("620080001611562C8802118E34"));
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
    }

    #[test]
    fn test_part1_example4() {
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn test_part1_example5() {
        // literal value
        assert_eq!(6, part1("D2FE28"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(875, part1(INPUT));
    }

    #[test]
    fn test_parse_literal() {
        let mut binary = Binary::from_hex("D2FE28");
        assert_eq!(
            Packet { version: 6, payload: Payload::Literal(2021) },
            parse_packet(&mut binary)
        );
    }
}