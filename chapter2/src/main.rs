const NUMBER_OF_MONTH: u32 = 12;
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
static WEEKDAYS: [&str; 7] = ["SUNDAY", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"];

fn print_months() {
    println!("Number of months in the year: {NUMBER_OF_MONTH}");
}

fn print_seasons() {
    for season in SEASONS {
        println!("Current season is {season}");
    }
}

fn print_weekdays() {
    let mut index = 0;

    while index < 7 {
        println!("Day: {} is {}", index + 1, WEEKDAYS[index]);
        index += 1;
    }
}

fn print_do_not_combine_immutable_and_mutable_reference(){
    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("{}", number_ref);
}
fn main() {
    let firstname = "Christopher";
    let lastname: String = "Monney".into();

    let name = format!("{firstname}, {lastname}");

    println!("Firstname: {} and Lastname: {} make the name: {}", firstname, lastname, name);

    print_months();
    print_seasons();
    print_weekdays();

    let mut my_number = 8;
    let my_num_ref = &mut my_number;
    *my_num_ref += 10;

    println!("My number is {}", my_number);

    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Are they equal? {}", second_number == ***triple_reference);

    print_do_not_combine_immutable_and_mutable_reference();
}
