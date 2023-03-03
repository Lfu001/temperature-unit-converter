use colored::Colorize;
use std::collections::HashSet;
use std::io;

fn main() {
    println!(
        "{}",
        "Welcome to temperature unit converter!"
            .bold()
            .bright_blue()
    );

    let degree_type = ask_degree_type();

    let temperature = ask_temperature();

    let converted_temperature = if degree_type.as_str() == "c" {
        c_to_f(temperature)
    } else {
        f_to_c(temperature)
    };

    let conversion_result = format!(
        "{} {} -> {} {}",
        temperature,
        degree_type_to_unit(&degree_type, false),
        converted_temperature,
        degree_type_to_unit(&degree_type, true)
    )
    .bold()
    .underline()
    .cyan();
    println!("\nResult: {}\n", conversion_result)
}

fn ask_degree_type() -> String {
    loop {
        println!(
            "{} {}\n{}",
            "How to convert temperature?".bright_green(),
            "(c/f)".blue(),
            "\tf: ˚F -> ˚C\n\tc: ˚C -> ˚F".blue()
        );

        let mut degree_type = String::new();
        io::stdin()
            .read_line(&mut degree_type)
            .expect(format!("{}", "Failed to read line.".red()).as_str());
        degree_type = degree_type.trim().to_lowercase();

        let degree_type = match verify_degree_type(&degree_type) {
            true => degree_type,
            false => {
                println!(
                    "{}",
                    "invalid degree type: String in [\"c\", \"f\"] expected.".red()
                );
                continue;
            }
        };

        return degree_type;
    }
}

fn ask_temperature() -> f64 {
    loop {
        println!("{}", "temperature value?".bright_green());

        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect(format!("{}", "Failed to read line.".red()).as_str());
        let temperature: f64 = match temperature.trim().parse() {
            Ok(t) => t,
            Err(_) => {
                println!("{}", "invalid temperature: float value expected.".red());
                continue;
            }
        };

        return temperature;
    }
}

fn verify_degree_type(degree_type: &String) -> bool {
    let all_type = HashSet::from(["c", "f"]);
    return all_type.contains(degree_type.as_str());
}

fn f_to_c(t_f: f64) -> f64 {
    return (t_f - 32.0) * 5.0 / 9.0;
}

fn c_to_f(t_c: f64) -> f64 {
    return (t_c * 9.0 / 5.0) + 32.0;
}

fn degree_type_to_unit(degree_type: &String, invert: bool) -> String {
    let c_unit = String::from("˚C");
    let f_unit = String::from("˚F");

    if degree_type == "c" {
        return if invert { f_unit } else { c_unit };
    } else {
        return if invert { c_unit } else { f_unit };
    }
}
