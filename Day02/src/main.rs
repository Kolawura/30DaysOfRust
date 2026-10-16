fn main() {
    let temp = temp_converter(30.0,"C","K");
    println!("Converted temperature: {}", temp);
}

fn temp_converter (temp:f64, unit:&str, target_unit: &str)->f64 {
    let converted_temp = match (unit, target_unit) {
        ("C", "F") => (temp * 9.0 / 5.0) + 32.0,
        ("F", "C") => (temp - 32.0) * 5.0 / 9.0,
        ("K", "C") => temp - 273.15,
        ("C", "K") => temp + 273.15,
        ("K", "F") => (temp - 273.15) * 9.0 / 5.0 + 32.0,
        ("F", "K") => (temp - 32.0) * 5.0 / 9.0 + 273.15,
        _ => panic!("Invalid unit conversion"),
    };
    return converted_temp;
}


