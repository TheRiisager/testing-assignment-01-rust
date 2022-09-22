use std::collections::HashMap;

pub fn arabic_to_roman(number: i32) -> String {
    if number < 0 || number > 3999 {
        return "".to_string();
    }

    let ones: HashMap<i32, &str> = HashMap::from([
        (0, ""),
        (1, "I"),
        (2, "II"),
        (3, "III"),
        (4, "IV"),
        (5, "V"),
        (6, "VI"),
        (7, "VII"),
        (8, "VIII"),
        (9, "IX")
    ]);
    let tens: HashMap<i32, &str> = HashMap::from([
        (0, ""),
        (1, "X"),
        (2, "XX"),
        (3, "XXX"),
        (4, "XL"),
        (5, "L"),
        (6, "LX"),
        (7, "LXX"),
        (8, "LXXX"),
        (9, "XC")
    ]);
    let hundreds: HashMap<i32, &str> = HashMap::from([
        (0, ""),
        (1, "C"),
        (2, "CC"),
        (3, "CCC"),
        (4, "CD"),
        (5, "D"),
        (6, "DC"),
        (7, "DCC"),
        (8, "DCCC"),
        (9, "CM")
    ]);
    let thousands: HashMap<i32, &str> = HashMap::from([
        (0, ""),
        (1, "M"),
        (2, "MM"),
        (3, "MMM"),
    ]);

    let mut roman = String::new();
    let mut num = number;

    //ones
    let roman_ones = ones.get(&(num % 10)).unwrap();
    num /= 10;

    //tens
    let mut roman_tens = "";
    if number > 9 {
        roman_tens = tens.get(&(num % 10)).unwrap();
        num /= 10;
    }

    //hundreds
    let mut roman_hundreds = "";
    if number > 99 {
        roman_hundreds = hundreds.get(&(num % 10)).unwrap();
        num /= 10;
    }

    //thousands
    let mut roman_thousands = "";
    if number > 999 {
        roman_thousands = thousands.get(&(num % 10)).unwrap();
    }

    roman.push_str(roman_thousands);
    roman.push_str(roman_hundreds);
    roman.push_str(roman_tens);
    roman.push_str(roman_ones);

    return roman;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_to_roman_1() {
        let roman = arabic_to_roman(1);
        assert_eq!(roman, "I");
    }

    #[test]
    fn test_arabic_to_roman_10() {
        let roman = arabic_to_roman(10);
        assert_eq!(roman, "X");
    }

    #[test]
    fn test_arabic_to_roman_100() {
        let roman = arabic_to_roman(100);
        assert_eq!(roman, "C");
    }

    #[test]
    fn test_arabic_to_roman_1000() {
        let roman = arabic_to_roman(1000);
        assert_eq!(roman, "M");
    }

    #[test]
    fn test_arabic_to_roman_1234() {
        let roman = arabic_to_roman(1234);
        assert_eq!(roman, "MCCXXXIV");
    }

    #[test]
    fn test_arabic_to_roman_out_of_bounds() {
        let roman = arabic_to_roman(9999);
        assert_eq!(roman, "");
    }
}