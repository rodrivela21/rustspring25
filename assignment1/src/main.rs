const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() {
    let temperature_f = 32.0;

    let temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{:.2}°F is {:.2}°C", temperature_f, temperature_c);

    for i in 1..=5 {
        let next_f = temperature_f + i as f64;
        let next_c = fahrenheit_to_celsius(next_f);
        println!("{:.2}°F is {:.2}°C", next_f, next_c);
    }
}
