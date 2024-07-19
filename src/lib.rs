// String
pub fn read_string(comment:&str) -> String {
    println!("{}", comment);
    let mut string: String = String::new();

    std::io::stdin().read_line(&mut string)
        .ok()
        .expect("Error read line!");

    return string;
}

// i8 - i128
pub fn read_i8(comment:&str) -> i8 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<i8>().unwrap();
}

pub fn read_i16(comment:&str) -> i16 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<i16>().unwrap();
}

pub fn read_i32(comment:&str) -> i32 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<i32>().unwrap();
}

pub fn read_i64(comment:&str) -> i64 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<i64>().unwrap();
}

pub fn read_i128(comment:&str) -> i128 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<i128>().unwrap();
}

// u8 - u128
pub fn read_u8(comment:&str) -> u8 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<u8>().unwrap();
}

pub fn read_u16(comment:&str) -> u16 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<u16>().unwrap();
}

pub fn read_u32(comment:&str) -> u32 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<u32>().unwrap();
}

pub fn read_u64(comment:&str) -> u64 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<u64>().unwrap();
}

pub fn read_u128(comment:&str) -> u128 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<u128>().unwrap();
}

// f32 and f64
pub fn read_f32(comment:&str) -> f32 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<f32>().unwrap();
}

pub fn read_f64(comment:&str) -> f64 {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<f64>().unwrap();
}

// char 
pub fn read_char(comment:&str) -> char {
    println!("{}", comment);
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Error read line!");

    return input.trim().parse::<char>().unwrap();
}