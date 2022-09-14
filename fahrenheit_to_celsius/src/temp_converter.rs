
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    let celsius = (fahrenheit - 32.0) / 1.8;
    let rounded = (celsius * 100.0).round() / 100.0;
    return rounded;
}

pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    let fahrenheit = (celsius * 1.8) + 32.0;
    let rounded = (fahrenheit * 100.0).round() / 100.0;
    return rounded;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    //fahrenheit to celsius tests
    #[test]
    fn test_fahrenheit_to_celsius_20deg() {
        let fahrenheit = 20.0;

        assert_eq!(fahrenheit_to_celsius(fahrenheit), -6.67);
    }

    #[test]
    fn test_fahrenheit_to_celsius_10deg() {
        let fahrenheit = 10.0;

        assert_eq!(fahrenheit_to_celsius(fahrenheit), -12.22);
    }

    #[test]
    fn test_fahrenheit_to_celsius_0deg() {
        let fahrenheit = 0.0;

        assert_eq!(fahrenheit_to_celsius(fahrenheit), -17.78);
    }

    //Celsius to fahrenheit tests
    #[test]
    fn test_celsius_to_fahrenheit_20deg() {
        let celsius = 20.0;

        assert_eq!(celsius_to_fahrenheit(celsius), 68.00);
    }

    #[test]
    fn test_celsius_to_fahrenheit_10deg() {
        let celsius = 10.0;

        assert_eq!(celsius_to_fahrenheit(celsius), 50.00);
    }

    #[test]
    fn test_celsius_to_fahrenheit_0deg() {
        let celsius = 0.0;

        assert_eq!(celsius_to_fahrenheit(celsius), 32.00);
    }
}