pub(crate) fn convert_fahrenheit_to_celsius() {
    let mut fahrenheit_degree: f32 = 40.0;
    let celsius_degree = (fahrenheit_degree - 32.0) / 1.8;
    println!("Fahrenheit degree: {fahrenheit_degree}");
    println!("Celsius degree: {celsius_degree}");
}
