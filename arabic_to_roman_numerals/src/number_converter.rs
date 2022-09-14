use std::collections::HashMap;

pub fn arabic_to_roman(number: i32) -> String {

    if number < 0 || number > 3999 {
        return "".to_string();
    }

    let ones: HashMap<char, &str> = HashMap::from([
        ('0', ""),
        ('1', "I"),
        ('2', "II"),
        ('3', "III"),
        ('4', "IV"),
        ('5', "V"),
        ('6', "VI"),
        ('7', "VII"),
        ('8', "VIII"),
        ('9', "IX")
    ]);
    
    let tens: HashMap<char, &str> = HashMap::from([
        ('0', ""),
        ('1', "X"),
        ('2', "XX"),
        ('3', "XXX"),
        ('4', "XL"),
        ('5', "L"),
        ('6', "LX"),
        ('7', "LXX"),
        ('8', "LXXX"),
        ('9', "XC")
    ]);
    
    let hundreds: HashMap<char, &str> = HashMap::from([
        ('0', ""),
        ('1', "C"),
        ('2', "CC"),
        ('3', "CCC"),
        ('4', "CD"),
        ('5', "D"),
        ('6', "DC"),
        ('7', "DCC"),
        ('8', "DCCC"),
        ('9', "CM")
    ]);
    
    let thousands: HashMap<char, &str> = HashMap::from([
        ('0', ""),
        ('1', "M"),
        ('2', "MM"),
        ('3', "MMM"),
    ]);

    let mut roman: String = "".to_string();
    for (i, char) in number.to_string().chars().rev().enumerate() {
        match i {
            0 => {
                match ones.get(&char) {
                    Some(numeral) => {
                        roman += numeral;
                    }
                    _ => {}
                }
            }
            1 => {
                match tens.get(&char) {
                    Some(numeral) => {
                        roman += numeral;
                    }
                    _ => {}
                }
            }
            2 => {
                match hundreds.get(&char) {
                    Some(numeral) => {
                        roman += numeral;
                    }
                    _ => {}
                }
            }
            3 => {
                match thousands.get(&char) {
                    Some(numeral) => {
                        roman += numeral;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    println!("{}", roman);
    return roman.chars().rev().collect::<String>();
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