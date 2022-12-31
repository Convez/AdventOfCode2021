use std::str::Chars;


fn convert_to_binary_from_hex(hex: &str) -> String {
    hex[0..].chars().map(to_binary).collect()
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
#[derive(Debug,PartialEq,Eq,Clone, Copy)]
enum Operation{
    Literal,
    Other,
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal
}
#[derive(Debug,PartialEq, Eq,Clone)]
enum Message{
    Literal((u64,usize,Operation,String)),
    Operator((u64,usize,Operation,Vec<Message>))
}
fn get_next_chunk(chars: &mut Chars,amount:usize)->String{
    let mut chunk = "".to_string();
    for _ in 0..amount{
        chunk.push(chars.next().unwrap());
    }
    chunk
}

fn parse_message(chars: &mut Chars)->Message{
    let mut message_size = 0;
    let version = u64::from_str_radix(get_next_chunk(chars,3).as_str(),2).unwrap();
    let m_type = u64::from_str_radix(get_next_chunk(chars,3).as_ref(),2).unwrap();
    message_size += 6;
    let operation = match m_type{
        4 => Operation::Literal,
        0 => Operation::Sum,
        1 => Operation::Product,
        2 => Operation::Min,
        3 => Operation::Max,
        5 => Operation::Greater,
        6 => Operation::Less,
        7 => Operation::Equal,
        _=> Operation::Other
    };
    if operation == Operation::Literal{
        let mut literal = "".to_string();
        loop {
            let flag = chars.next().unwrap();
            literal.push_str(&get_next_chunk(chars, 4));
            message_size += 5;
            if flag == '0'{
                break;
            }
        }
        // Align with hex encoding
        return Message::Literal((version,message_size,Operation::Literal,u64::from_str_radix(&literal, 2).unwrap().to_string()));
    }else{
        let label = chars.next().unwrap();
        message_size+=1;
        if label == '0'{
            let sub_packets_length = u64::from_str_radix(get_next_chunk(chars, 15).as_str(),2).unwrap();
            message_size+=15;
            let mut sub_packets = vec![];
            loop {
                let sub_packet = parse_message(chars);
                sub_packets.push(sub_packet);
                let curr_len = sub_packets.iter().map(|p|{
                    if let Message::Literal(m)=p{
                        return m.1
                    }
                    if let Message::Operator(m)=p{
                        return m.1
                    }
                    return 0
                }).sum::<usize>();
                assert!(curr_len<=sub_packets_length as usize);
                if curr_len == sub_packets_length as usize{
                    break;
                }
            }
            message_size+=sub_packets_length as usize;
            return Message::Operator((version,message_size,operation,sub_packets));
        }else{
            let sub_packets_number = u64::from_str_radix(get_next_chunk(chars, 11).as_str(),2).unwrap();
            message_size+=11;
            let mut sub_packets = vec![];
            loop {
                let sub_packet = parse_message(chars);
                sub_packets.push(sub_packet);
                if sub_packets.len() == sub_packets_number as usize{
                    break;
                }
            }
            let curr_len = sub_packets.iter().map(|p|{
                if let Message::Literal(m)=p{
                    return m.1
                }
                if let Message::Operator(m)=p{
                    return m.1
                }
                return 0
            }).sum::<usize>();
            message_size+=curr_len;
            return Message::Operator((version,message_size,operation,sub_packets));
        }
    }
}

fn sum_versions(message:&Message)->u64{
    if let Message::Literal(m) = message{
        return m.0;
    }
    if let Message::Operator(m) = message{
        return m.0 + m.3.iter().map(|un|sum_versions(un)).sum::<u64>();
    }
    0
}
fn perform_operation(message:&Message)->u64{
    if let Message::Literal(m) = message{
        return m.3.parse::<u64>().unwrap();
    }
    if let Message::Operator(m) = message{
        match m.2 {
            Operation::Sum => return m.3.iter().map(|sp|perform_operation(sp)).sum(),
            Operation::Product => return m.3.iter().map(|sp|perform_operation(sp)).reduce(|acc,it|acc*it).unwrap(),
            Operation::Min => return m.3.iter().map(|sp|perform_operation(sp)).min().unwrap(),
            Operation::Max => return m.3.iter().map(|sp|perform_operation(sp)).max().unwrap(),
            Operation::Greater => if perform_operation(&m.3[0])>perform_operation(&m.3[1]){return 1;} else {return 0;},
            Operation::Less => if perform_operation(&m.3[0])<perform_operation(&m.3[1]){return 1;} else {return 0;},
            Operation::Equal => if perform_operation(&m.3[0])==perform_operation(&m.3[1]){return 1;} else {return 0;},
            Operation::Literal => todo!(),
            Operation::Other => todo!(),
        }
    }
    return 0;
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    for line in input.lines(){
        let bin = convert_to_binary_from_hex(line.trim());
        let mut chars = bin.chars();
        let message = parse_message(&mut chars);
        println!("Sum of versions for string {} is {}", line.trim(),sum_versions(&message));
        println!("The result of the operation for the same packet is: {}", perform_operation(&message));
    }
}