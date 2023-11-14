/// Shadowing doesn't destroy a value but blocks it
/// The following code proves it with the use of references
fn shadowing_example() {
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{country_ref} {country}");
}

/// Giving references to functions
/// Value can only have one owner
fn print_country(country_name: &String) {
    println!("Country name: {country_name}");
}

fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now it says: {country_name}");
}

fn print_binary_hexadecimal_octal() {
    let number = 555;
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", number, number, number);
}

fn print_variables_in_positions() {
    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Tepes";
    println!("This is {1} {2}, son of {0} {2}", father_name, son_name, family_name);
}

fn main() {
    let mut country = String::from("Austria");
    println!("{:p}", &country);
    print_country(&country);
    print_country(&country);
    add_hungary(&mut country);
    print_binary_hexadecimal_octal();
    print_variables_in_positions();
    println!("{city1} is in {country} and {city2} is also in {country}, but {city3} is not in {country}",
             city1 = "Seoul", city2 = "Busan", city3 = "Tokyo", country = "Korea");

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
