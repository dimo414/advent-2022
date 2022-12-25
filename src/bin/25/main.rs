use anyhow::*;

fn main() -> Result<()> {
    let input = parse_input(include_str!("input.txt"))?;
    let sum: i64 = input.iter().sum();
    println!("Fuel requirements: {} ({})", to_snafu(sum), sum);

    Ok(())
}

fn from_snafu(snafu: &str) -> Result<i64> {
    let mut num = 0;
    for c in snafu.chars() {
        num *= 5;
        match c {
            '2' => num += 2,
            '1' => num += 1,
            '0' => {},
            '-' => num -= 1,
            '=' => num -= 2,
            _ => { bail!("Invalid: {}", c); },
        }
    }
    Ok(num)
}

fn to_snafu(dec: i64) -> String {
    assert!(dec > 0);
    let mut dec = dec;
    let mut ret = Vec::new();
    while dec > 0 {
        let cur = dec % 5;
        match cur {
            0|1|2 => ret.push(char::from_digit(cur as u32, 3).expect("Must be digit")),
            3 => {
                ret.push('=');
                dec += 2;
            },
            4 => {
                ret.push('-');
                dec += 1;
            },
            _ => unreachable!(),
        }
        dec /= 5;
    }
    ret.iter().rev().collect()
}

fn parse_input(input: &str) -> Result<Vec<i64>> {
    input.lines().map(from_snafu).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { parse_input(include_str!("input.txt")).unwrap(); }

    parameterized_test::create!{ brochure, (dec, snafu), {
        assert_eq!(dec, from_snafu(snafu).unwrap());
        assert_eq!(to_snafu(dec), snafu);
    } }
    brochure! {
        a: (1, "1"),
        b: (2, "2"),
        c: (3, "1="),
        d: (4, "1-"),
        e: (5, "10"),
        f: (6, "11"),
        g: (7, "12"),
        h: (8, "2="),
        i: (9, "2-"),
        j: (10, "20"),
        k: (15, "1=0"),
        l: (20, "1-0"),
        m: (2022, "1=11-2"),
        n: (12345, "1-0---0"),
        o: (314159265, "1121-1110-1=0"),
    }

    #[test]
    fn example() {
        let example = parse_input(include_str!("example.txt")).unwrap();
        let sum: i64 = example.iter().sum();
        assert_eq!(sum, 4890);
        assert_eq!(to_snafu(sum), "2=-1=0");
    }
}
