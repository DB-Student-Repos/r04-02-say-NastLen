pub fn encode(n: u64) -> String {
    let ones = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"
    ];
    let tens = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
    ];

    if n < 20 {
        return ones[n as usize].to_string();
    } else if n < 100 {
        let mut result = tens[(n / 10 - 2) as usize].to_string();
        if n % 10 != 0 {
            result.push_str("-");
            result.push_str(ones[(n % 10) as usize]);
        }
        return result;
    } else if n < 1000 {
        let mut result = ones[(n / 100) as usize].to_string();
        result.push_str(" hundred");
        if n % 100 != 0 {
            result.push_str(" and ");
            result.push_str(&encode(n % 100));
        }
        return result;
    } else if n < 1000000 {
        let mut result = encode(n / 1000);
        result.push_str(" thousand");
        if n % 1000 != 0 {
            result.push_str(" ");
            result.push_str(&encode(n % 1000));
        }
        return result;
    } else if n < 1000000000 {
        let mut result = encode(n / 1000000);
        result.push_str(" million");
        if n % 1000000 != 0 {
            result.push_str(" ");
            result.push_str(&encode(n % 1000000));
        }
        return result;
    } else if n < 1000000000000 {
        let mut result = encode(n / 1000000000);
        result.push_str(" billion");
        if n % 1000000000 != 0 {
            result.push_str(" ");
            result.push_str(&encode(n % 1000000000));
        }
        return result;
    } else {
        panic!("Number too large");
    }
}
