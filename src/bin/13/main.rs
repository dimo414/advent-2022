use std::cmp::Ordering;
use std::str::FromStr;
use anyhow::{anyhow, ensure, Error, Result};

fn main() -> Result<()> {
    let input = parse_input(include_str!("input.txt")).unwrap();
    println!("Ordered packets sum: {}", sum_ordered(&input));

    println!("Dividers product: {}", dividers(&input));
    Ok(())
}

fn sum_ordered(packets: &[Packet]) -> usize {
    assert_eq!(packets.len() % 2, 0);
    packets.chunks(2).enumerate().filter(|(_, p)| p[0].cmp(&p[1]).is_lt()).map(|(i, _)| i+1).sum()
}

fn dividers(packets: &[Packet]) -> usize {
    let dividers = ["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    let mut packets: Vec<_> = packets.iter().chain(dividers.iter()).collect();
    packets.sort();
    packets.into_iter().enumerate()
        .filter(|(_, p)| p == &&dividers[0] || p == &&dividers[1])
        .map(|(i, _)| i+1)
        .product()
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn num_to_list(n: i32) -> Packet {
        Packet::List(vec![Packet::Num(n)])
    }

    #[cfg(test)]
    fn list(nums: &[i32]) -> Packet {
        Packet::List(nums.iter().map(|n| Packet::Num(*n)).collect())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match self {
            Num(s) =>
                match other {
                    Num(o) => s.cmp(o),
                    List(_) => Packet::num_to_list(*s).cmp(other),
                },
            List(s) =>
                match other {
                    Num(o) => self.cmp(&Packet::num_to_list(*o)),
                    List(o) => {
                        for i in 0..(std::cmp::min(s.len(), o.len())) {
                            let cmp = s[i].cmp(&o[i]);
                            if cmp != Ordering::Equal {
                                return cmp;
                            }
                        }
                        s.len().cmp(&o.len())
                    },
                },
        }
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn partial(s: &[u8]) -> Result<(Packet, usize)> {
            ensure!(!s.is_empty(), "Empty");
            let mut idx = 0;
            while idx < s.len() && (s[idx] as char).is_ascii_digit() {
                idx += 1;
            }
            if idx > 0 { // digit found
                return Ok((Packet::Num(String::from_utf8_lossy(&s[0..idx]).parse::<i32>()?), idx));
            }
            ensure!(s[idx] == b'[');
            idx += 1;
            let mut vec = Vec::new();
            loop {
                if s[idx] == b']' { break; }
                let (elem, next) = partial(&s[idx..])?;
                vec.push(elem);
                idx += next;
                if s[idx] == b']' { break; }
                ensure!(s[idx] == b',', "{:?}", String::from_utf8_lossy(&s[idx..]));
                idx += 1;
            }
            Ok((Packet::List(vec), idx + 1))
        }

        let (ret, len) = partial(s.as_bytes())?;
        ensure!(len == s.len(), "Incomplete parse: {:?}", &s[len..]);
        Ok(ret)
    }
}

fn parse_input(input: &str) -> Result<Vec<Packet>> {
    fn parse_pair(entry: &str) -> Vec<Result<Packet>> {
        let parts = entry.lines().map(|l| l.parse()).collect::<Vec<_>>();
        if parts.len() != 2 {
            return vec![Err(anyhow!("Invalid entry: {:?}", entry))];
        }
        parts
    }
    input.split("\n\n").flat_map(|e| parse_pair(e).into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { parse_input(include_str!("input.txt")).unwrap(); }

    parameterized_test::create!{ parse, (s, expected), { assert_eq!(s.parse::<Packet>()?, expected); } }
    parse! {
        num: ("10", Packet::Num(10)),
        singleton: ("[3]", Packet::num_to_list(3)),
        list: ("[1,1,3,1,1]", Packet::list(&[1,1,3,1,1])),
        nest: ("[[1],[2,3,4]]", Packet::List(vec![Packet::list(&[1]), Packet::list(&[2,3,4])])),
        wrapped: ("[[8,7,6]]", Packet::List(vec![Packet::list(&[8,7,6])])),
        empties: ("[[[]]]", Packet::List(vec![Packet::List(vec![Packet::List(vec![])])])),
    }

    parameterized_test::create!{ compare, (left, right, ordering), {
        let left = left.parse::<Packet>()?;
        let right = right.parse::<Packet>()?;
        assert_eq!(left.cmp(&right), ordering);
    } }
    compare! {
        a: ("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less),
        b: ("[[1],[2,3,4]]", "[[1],4]", Ordering::Less),
        c: ("[9]", "[[8,7,6]]", Ordering::Greater),
        d: ("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less),
        e: ("[7,7,7,7]", "[7,7,7]", Ordering::Greater),
        f: ("[]", "[3]", Ordering::Less),
        g: ("[[[]]]", "[[]]", Ordering::Greater),
        h: ("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", Ordering::Greater),
        equal: ("[[4,4],4,4]", "[[4,4],4,4]", Ordering::Equal),
    }

    #[test]
    fn example() {
        let input = parse_input(include_str!("example.txt")).unwrap();
        assert_eq!(sum_ordered(&input), 13);
        assert_eq!(dividers(&input), 140);
    }
}
