use std::io::stdin;

fn main() {
    loop {
        println!("Celsius <-> Fahrenheit converter");
        let mut temp = String::new();
        let mut to_unit = String::new();

        println!("Please input the temperature you want to convert to: ");
        match stdin().read_line(&mut to_unit) {
            Ok(value) => value,
            Err(_) => break,
        };

        let to_unit = to_unit.trim();

        if to_unit != "celsius" && to_unit != "fahrenheit" {
            println!("Unit to convert to must be Celsius or Fahrenheit: ");
            break;
        }

        let input_temp = if to_unit == "celsius" {
            "Fahrenheit"
        } else {
            "Celsius"
        };

        println!("Please input the {} temperature value", input_temp);
        match stdin().read_line(&mut temp) {
            Ok(value) => value,
            Err(_) => break,
        };

        let temp: i32 = match temp.trim().parse() {
            Ok(value) => value,
            Err(_) => break,
        };

        println!("{}", convert_fah_cel(to_unit, temp));
    }
}

#[allow(dead_code, unused_variables)]
fn convert_fah_cel(to_unit: &str, value: i32) -> String {
    if to_unit == "celsius" {
        return to_cel(value);
    } else {
        return to_fah(value);
    }
}

#[allow(dead_code, unused_variables)]
fn to_cel(value: i32) -> String {
    // Cel = 5/9(Fah - 32)
    let value: f64 = f64::from(value);
    let cel: f64 = (5.0 / 9.0) * (value - 32.0);
    let mut cel: String = cel.to_string();
    cel.push_str(" Cel");
    return cel;
}

#[allow(dead_code, unused_variables)]
fn to_fah(value: i32) -> String {
    // Fah = 9/5(Cel) + 32
    let value: f64 = f64::from(value);
    let fah: f64 = ((9.0 / 5.0) * value) + 32.0;
    let mut fah: String = fah.to_string();
    fah.push_str(" Fah");
    return fah;
}
