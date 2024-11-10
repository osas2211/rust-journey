use std::io::stdin;
#[derive(Debug)]
struct Temperature {
    to_unit: String,
    value: String,
}

impl Temperature {
    fn convert_fah_cel(self) -> String {
        if self.to_unit.trim() == "celsius" {
            self.to_cel()
        } else {
            self.to_fah()
        }
    }

    fn to_cel(self) -> String {
        let value: f64 = f64::from(self.int_value());
        let cel: f64 = (5.0 / 9.0) * (value - 32.0);
        let mut cel: String = cel.to_string();
        cel.push_str(" Cel");
        return cel;
    }

    fn to_fah(self) -> String {
        // Fah = 9/5(Cel) + 32
        let value: f64 = f64::from(self.int_value());
        let fah: f64 = ((9.0 / 5.0) * value) + 32.0;
        let mut fah: String = fah.to_string();
        fah.push_str(" Fah");
        return fah;
    }

    fn int_value(self) -> u32 {
        let value = self
            .value
            .trim()
            .parse()
            .expect("Please input a valid number");
        value
    }
}

fn main() {
    loop {
        let mut fah_cel = Temperature {
            to_unit: String::new(),
            value: String::new(),
        };
        println!("Enter the unit you want to convert to: (celsius or fahrenheit or quit to End: )");
        match stdin().read_line(&mut fah_cel.to_unit) {
            Ok(value) => {
                println!("{}", value.to_string());
                if String::from(value.to_string()).trim() == "quit" {
                    break;
                }
                value
            }
            Err(_) => {
                println!("Please input a value");
                continue;
            }
        };

        println!("Enter the value: ");
        match stdin().read_line(&mut fah_cel.value) {
            Ok(value) => value,
            Err(_) => {
                println!("Please input a valid value");
                continue;
            }
        };

        println!("{}", fah_cel.convert_fah_cel());
    }
}
