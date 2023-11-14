fn array_example() {
    let my_array = ["a"; 5];
    println!("{:?}", my_array);

    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let two_to_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!("Two to five: {0:?}, Start at one: {1:?}, End at five: {2:?}, Everything: {3:?}",
             two_to_five, start_at_one, end_at_five, everything);
}

fn vectors_example() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();

    my_vec.push(name1);
    my_vec.push(name2);
}

fn vec_macro_example(){
    let vec_of_ten = vec![1,2,3,4,5,6,7,8,9,10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("Three to five: {0:?}, Start at two: {1:?}, End at five: {2:?}, Everything: {3:?}",
             three_to_five, start_at_two, end_at_five, everything);
}

fn main() {
    array_example();
    vectors_example();
    vec_macro_example();
}
