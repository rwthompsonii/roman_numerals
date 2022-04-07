fn roman_helper(one: char, five: char, ten: char, value: isize) -> String {
    match value {
        0 => "".to_string(),
        i @ (1 | 2 | 3) => (0..i).fold("".to_string(), |mut str, _| {
            str.push(one);
            str
        }),
        4 => format!("{one}{five}"),
        5 => five.to_string(),
        i @ (6 | 7 | 8) => (5..i).fold(five.to_string(), |mut str, _| {
            str.push(one);
            str
        }),
        9 => format!("{one}{ten}"),
        _ => unreachable!(),
    }
}

pub fn to_roman(number: isize) -> Result<String, String> {
    if number <= 0 || number > 3000 {
        return Err(format!(
            "invalid input : {number} must be in range 1 < number <= 3000."
        ));
    }

    let thousands = number / 1000;
    let number = number - (thousands * 1000);
    let hundreds = number / 100;
    let number = number - (hundreds * 100);
    let tens = number / 10;
    let ones = number - (tens * 10);

    let thousands_str = match thousands {
        0 => "".to_string(),
        i @ (1 | 2 | 3) => (0..i).fold("".to_string(), |mut str, _| {
            str.push('M');
            str
        }),
        _ => unreachable!(), //asserted <= 3000
    };

    let hundreds_str = roman_helper('C', 'D', 'M', hundreds);
    let tens_str = roman_helper('X', 'L', 'C', tens);
    let ones_str = roman_helper('I', 'V', 'X', ones);

    Ok(format!("{thousands_str}{hundreds_str}{tens_str}{ones_str}"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_i() {
        assert_eq!(to_roman(1).unwrap(), "I");
    }

    #[test]
    pub fn test_iii() {
        assert_eq!(to_roman(3).unwrap(), "III");
    }

    #[test]
    pub fn test_iv() {
        assert_eq!(to_roman(4).unwrap(), "IV");
    }

    #[test]
    pub fn test_v() {
        assert_eq!(to_roman(5).unwrap(), "V");
    }

    #[test]
    pub fn test_viii() {
        assert_eq!(to_roman(8).unwrap(), "VIII");
    }

    #[test]
    pub fn test_ix() {
        assert_eq!(to_roman(9).unwrap(), "IX");
    }

    #[test]
    pub fn test_x() {
        assert_eq!(to_roman(10).unwrap(), "X");
    }

    #[test]
    pub fn test_xxxvii() {
        assert_eq!(to_roman(37).unwrap(), "XXXVII");
    }

    #[test]
    pub fn test_lxiv() {
        assert_eq!(to_roman(64).unwrap(), "LXIV");
    }

    #[test]
    pub fn test_lxxxix() {
        assert_eq!(to_roman(89).unwrap(), "LXXXIX");
    }

    #[test]
    pub fn test_cxxiii() {
        assert_eq!(to_roman(123).unwrap(), "CXXIII");
    }

    #[test]
    pub fn test_cdlvi() {
        assert_eq!(to_roman(456).unwrap(), "CDLVI");
    }

    #[test]
    pub fn test_dlv() {
        assert_eq!(to_roman(555).unwrap(), "DLV");
    }

    #[test]
    pub fn test_dclxxviii() {
        assert_eq!(to_roman(678).unwrap(), "DCLXXVIII");
    }

    #[test]
    pub fn test_mccxxxiv() {
        assert_eq!(to_roman(1234).unwrap(), "MCCXXXIV");
    }

    #[test]
    pub fn test_mmccxlv() {
        assert_eq!(to_roman(2345).unwrap(), "MMCCCXLV");
    }

    #[test]
    pub fn test_mmm() {
        assert_eq!(to_roman(3000).unwrap(), "MMM");
    }

    #[test]
    pub fn test_mmmi() {
        assert!(to_roman(3001).is_err());
    }

    #[test]
    pub fn test_zero() {
        assert!(to_roman(0).is_err());
    }

    #[test]
    pub fn test_negative() {
        assert!(to_roman(-1).is_err());
    }
}
