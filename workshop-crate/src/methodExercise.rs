struct Fahrenheit {
    temp: f64,
}

struct Celsius {
    temp: f64,
}

impl Celsius {
    fn c_to_f(&self) -> Fahrenheit {
        Fahrenheit {
            temp: self.temp * 1.8 + 32.0,
        }
    }
}

fn f_to_c(temp_f: Fahrenheit) -> Celsius {
    return Celsius {
        temp: (temp_f.temp - 32.0) / 1.8
    };
}

fn main() {

    let f_temp: Fahrenheit = Fahrenheit {
        temp: 100.0,
    };

    let c_temp: Celsius = f_to_c(f_temp);

    println!("{}", c_temp.temp);
    println!("{}", c_temp.c_to_f().temp);
}

