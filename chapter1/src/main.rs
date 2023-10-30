fn main() {
    let my_number = 600;
    println!("{}", my_number as u8);
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());

    let small_number_1: u8 = 10;
    let small_number_2 = 10u8;
    let small_number_3 = 10_u8;

    println!("number 1: {}, number 2: {}, number 3: {}", small_number_1, small_number_2, small_number_3);

    let some_number = {
        let second_number = 8;
        second_number + 9
    };

    println!("Some number is: {:?}", some_number);

    println!("The smallest i8: {} The largest i8: {}", i8::MIN, i8::MAX);
    println!("The smallest u8: {} The largest u8: {}", u8::MIN, u8::MAX);
    println!("The smallest i16: {} The largest i16: {}", i16::MIN, i16::MAX);
    println!("The smallest u16: {} The largest u16: {}", u16::MIN, u16::MAX);
    println!("The smallest i32: {} The largest i32: {}", i32::MIN, i32::MAX);
    println!("The smallest u32: {} The largest u32: {}", u32::MIN, u32::MAX);
    println!("The smallest i64: {} The largest i64: {}", i64::MIN, i64::MAX);
    println!("The smallest u64: {} The largest u64: {}", u64::MIN, u64::MAX);
    println!("The smallest i128: {} The largest i128: {}", i128::MIN, i128::MAX);
    println!("The smallest u128: {} The largest u128: {}", u128::MIN, u128::MAX);

    println!("{}", my_number);
    {
        let my_number = 9.2;
        println!("{}", my_number);
    }
    println!("{}", my_number);
}
