use took::Timer;

fn to_binary(x : u8) -> Vec<u8> {
    match x {
        b'0' => vec![0,0,0,0],
        b'1' => vec![0,0,0,1],
        b'2' => vec![0,0,1,0],
        b'3' => vec![0,0,1,1],
        b'4' => vec![0,1,0,0],
        b'5' => vec![0,1,0,1],
        b'6' => vec![0,1,1,0],
        b'7' => vec![0,1,1,1],
        b'8' => vec![1,0,0,0],
        b'9' => vec![1,0,0,1],
        b'A' => vec![1,0,1,0],
        b'B' => vec![1,0,1,1],
        b'C' => vec![1,1,0,0],
        b'D' => vec![1,1,0,1],
        b'E' => vec![1,1,1,0],
        b'F' => vec![1,1,1,1],
        _ => unreachable!()
    }
}

pub fn part_one() {
    let timer = Timer::new();
    
    let input : Vec<_> = include_bytes!("../inputs/day16.input").to_vec().iter()
        .map(|x| to_binary(*x))
        .flatten()
        .collect();

    let (version_sum, _, _) = process_packet(&input, 0);
    
    println!("day 16, part 1: {} in time {}us", version_sum, timer.took().as_std().as_micros());
}

fn map_bits_to_num(packet : &Vec<u8>, start_bit : usize, end_bit : usize) -> usize {
    let mut len = end_bit - start_bit;
    let mut result: usize = 0;
    for i in start_bit .. end_bit {
        result += (packet[i] as usize) << (len-1);
        len -= 1;
    }
    return result;
}

fn process_literal(packet : &Vec<u8>, mut next_bit: usize) -> (usize, usize, usize) {
    let mut result : usize = 0;
    
    loop {
        let v = map_bits_to_num(&packet, next_bit, next_bit+5);
        result = result << 4;
        result += (v & 0xF) as usize;
        next_bit += 5;
        if v >> 4 == 0 {
            break;
        }
    }

    return (0, next_bit, result);
}

fn process_operator(packet : &Vec<u8>, op_code : usize, mut next_bit : usize) -> (usize, usize, usize) {
    let length_type = packet[next_bit];
    next_bit += 1;
    let mut version_sum = 0;
    let mut operands : Vec<usize> = Vec::new();
    if length_type == 0 {
        let len = map_bits_to_num(packet, next_bit, next_bit+15);
        next_bit += 15;
        let end = next_bit + len;
        while next_bit < end {
            let mut version = 0;
            let mut result = 0;
            (version, next_bit, result) = process_packet(packet, next_bit);
            operands.push(result);
            version_sum += version;
        }
    } else if length_type == 1 {
        let len = map_bits_to_num(packet, next_bit, next_bit+11);
        next_bit += 11;
        for _ in 0 .. len {
            let mut version = 0;
            let mut result = 0;
            (version, next_bit, result) = process_packet(packet, next_bit);
            operands.push(result);
            version_sum += version;
        }
    }
    
    let result = match op_code {
        0 => operands.iter().fold(0, |acc, x| acc + x),
        1 => operands.iter().fold(1, |acc, x| acc * x),
        2 => *(operands.iter().min().unwrap()),
        3 => *(operands.iter().max().unwrap()),
        5 => if operands[0] > operands[1] { 1 } else { 0 }
        6 => if operands[0] < operands[1] { 1 } else { 0 }
        7 => if operands[0] == operands[1] { 1 } else { 0 }
        _ => 0
    };
    
    return (version_sum,next_bit,result);
}

fn process_packet(packet : &Vec<u8>, mut next_bit : usize) -> (usize, usize, usize) {
    
    let version : usize = map_bits_to_num(&packet, next_bit, next_bit + 3) as usize;
    next_bit += 3;
    
    let packet_type : usize = map_bits_to_num(&packet, next_bit, next_bit + 3) as usize;
    next_bit += 3;

    let mut version_sum = 0 + version;

    let (version, next_bit, result) = match packet_type {
        4 => process_literal(&packet, next_bit),
        _ => process_operator(&packet, packet_type,next_bit)
    };
    
    version_sum += version;
    
    return (version_sum, next_bit, result);
}


pub fn part_two() {
    let timer = Timer::new();
    
    let input : Vec<_> = include_bytes!("../inputs/day16.input").to_vec().iter()
        .map(|x| to_binary(*x))
        .flatten()
        .collect();

    let (version, _, result) = process_packet(&input, 0);

    println!("day 16, part 1: {}, part 2: {}, in time {}us", version, result, timer.took().as_std().as_micros());
}



pub fn part_two_streaming() {
    let timer = Timer::new();

    let mut input : Vec<_> = include_bytes!("../inputs/day16.input").to_vec().iter()
        .map(|x| to_binary(*x))
        .flatten()
        .collect();
    println!("parsing took: {}us", timer.took().as_std().as_micros());
    
    let (version_sum, result) = process_packet_optimized(&mut input);

    println!("day 16, part 1: {}, part 2: {}, in time {}us", version_sum, result, timer.took().as_std().as_micros());
}

fn process_packet_optimized(packet: &mut Vec<u8>) -> (usize, usize) {
    let version : usize = bits_to_num(packet.drain(0 .. 3)) as usize;
    let packet_type : usize = bits_to_num(packet.drain(0 .. 3)) as usize;
    
    let mut version_sum = 0 + version;

    let (version, result) = match packet_type {
        4 => process_literal_optimized(packet),
        _ => process_operator_optimized(packet, packet_type)
    };

    version_sum += version;

    return (version_sum, result);
    
}

fn bits_to_num(packet : impl IntoIterator<Item = u8>) -> usize {
    return packet.into_iter().fold(0, |acc, x| (acc << 1) + x as usize);
}

fn process_literal_optimized(packet : &mut Vec<u8>) -> (usize, usize) {
    let mut result : usize = 0;

    loop {
        let mut p = packet.drain(0 .. 5);
        let done = p.next().unwrap();
        let v = bits_to_num(p);
        result = result << 4;
        result += v;
        if done == 0 {
            break;
        }
    }

    return (0, result);
}

fn process_operator_optimized(packet : &mut Vec<u8>, op_code : usize) -> (usize, usize) {
    let length_type = packet.drain(0 .. 1).next().unwrap();
    let mut version_sum = 0;
    let mut operands : Vec<usize> = Vec::new();
    if length_type == 0 {
        let len = bits_to_num(packet.drain(0 .. 15));
        let mut the_rest = packet.drain(0 .. len).collect::<Vec<u8>>();
        while !the_rest.is_empty() {
            let mut version = 0;
            let mut result = 0;
            (version, result) = process_packet_optimized(&mut the_rest);
            operands.push(result);
            version_sum += version;
        }
    } else if length_type == 1 {
        let len = bits_to_num(packet.drain(0 .. 11));
        for _ in 0 .. len {
            let mut version = 0;
            let mut result = 0;
            (version, result) = process_packet_optimized(packet);
            operands.push(result);
            version_sum += version;
        }
    }

    let result = match op_code {
        0 => operands.iter().fold(0, |acc, x| acc + x),
        1 => operands.iter().fold(1, |acc, x| acc * x),
        2 => *(operands.iter().min().unwrap()),
        3 => *(operands.iter().max().unwrap()),
        5 => if operands[0] > operands[1] { 1 } else { 0 }
        6 => if operands[0] < operands[1] { 1 } else { 0 }
        7 => if operands[0] == operands[1] { 1 } else { 0 }
        _ => 0
    };

    return (version_sum, result);
}