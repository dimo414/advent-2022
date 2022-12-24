use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use anyhow::{Context, Error, Result};

fn main() -> Result<()> {
    let input: Encrypted = include_str!("input.txt").parse()?;
    let mut encr = input.clone();
    encr.mix();
    println!("Initial attempt: {}", encr.resolve_coordinates().iter().sum::<isize>());

    let mut encr = input;
    encr.apply_decryption();
    for _ in 0..10 { encr.mix(); }
    println!("Resolved coordinates: {}", encr.resolve_coordinates().iter().sum::<isize>());
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Element {
    index: usize,
    value: isize,
}

impl Element {
    fn create(index: usize, value: isize) -> Element {
        Element{ index, value }
    }
}

#[derive(Clone)]
struct Encrypted {
    data: Vec<Element>,
}

impl Encrypted {
    #[cfg(test)]
    fn create(data: &[isize]) -> Encrypted {
        Encrypted{ data:data.iter().enumerate().map(|(i,e)| Element::create(i, *e)).collect() }
    }

    fn apply_decryption(&mut self) {
        for e in self.data.iter_mut() {
            e.value *= 811589153;
        }
    }

    #[allow(clippy::comparison_chain)] // seems worse to swap to a match statement in this case
    fn mix_index(&mut self, original_index: usize) {
        let len = self.data.len();
        let rotate = len - 1; // data is cyclical so before[0] and after[len-1] are the same slot
        let cur_index = self.data[original_index].index;
        let offset = self.data[original_index].value;
        let new_index = ((((cur_index as isize + offset) % rotate as isize) + rotate as isize) % rotate as isize) as usize;
        debug_assert!((0..len).contains(&new_index));

        if new_index < cur_index {
            for elem in self.data.iter_mut() {
                if (new_index..=cur_index).contains(&elem.index) {
                    elem.index = ((len as isize + elem.index as isize + 1) % len as isize) as usize;
                    debug_assert!((0..len).contains(&elem.index));
                }
            }
        }
        else if cur_index < new_index {
            for elem in self.data.iter_mut() {
                if (cur_index..=new_index).contains(&elem.index) {
                    elem.index = ((len as isize + elem.index as isize - 1) % len as isize) as usize;
                    debug_assert!((0..len).contains(&elem.index));
                }
            }
        }
        self.data[original_index].index = new_index;
    }

    fn mix(&mut self) {
        for i in 0..self.data.len() {
            self.mix_index(i);
        }
    }

    fn make_vec(&self) -> Vec<isize> {
        let mut data = vec![None; self.data.len()];
        for e in &self.data {
            assert!(data[e.index].is_none(), "Collision! {:?} and {:?} at {}\n{:?}", data[e.index], e, e.index, self);
            data[e.index] = Some(e.value);
        }
        data.into_iter().map(|v| v.expect("All indexes")).collect()
    }

    fn make_zero_vec(&self) -> Vec<isize> {
        let data = self.make_vec();
        let zero = data.iter().position(|&e| e == 0).expect("Must contain 0");
        [&data[zero..], &data[..zero]].concat()
    }

    fn resolve_coordinates(&self) -> Vec<isize> {
        let zeroed = self.make_zero_vec();
        [1000, 2000, 3000].into_iter().map(|idx| zeroed[idx%zeroed.len()]).collect()
    }
}

impl PartialEq for Encrypted {
    fn eq(&self, other: &Self) -> bool {
        self.make_zero_vec() == other.make_zero_vec()
    }
}
impl Eq for Encrypted {}

impl Debug for Encrypted {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        for e in &self.data {
            write!(f, "{}:{} ", e.index, e.value)?;
        }
        write!(f, "}}")
    }
}

impl Display for Encrypted {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.make_vec())
    }
}

impl FromStr for Encrypted {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s.lines().enumerate()
            .map(|(i, e)| e.parse().map(|e| Element::create(i, e)).context(""))
            .collect::<Result<Vec<_>>>()?;
        Ok(Encrypted{data})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { include_str!("input.txt").parse::<Encrypted>().unwrap(); }

    #[test]
    fn example_pt1() {
        let mut encr = include_str!("example.txt").parse::<Encrypted>().unwrap();
        encr.mix();
        assert_eq!(encr, Encrypted::create(&[1, 2, -3, 4, 0, 3, -2]));
        assert_eq!(encr.resolve_coordinates(), &[4, -3, 2]);
    }

    #[test]
    fn example_pt2() {
        let mut encr = include_str!("example.txt").parse::<Encrypted>().unwrap();
        encr.apply_decryption();
        encr.mix();
        assert_eq!(encr, Encrypted::create(&[0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153]));
        for _ in 0..9 { encr.mix(); }
        assert_eq!(encr, Encrypted::create(&[0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153]));
        assert_eq!(encr.resolve_coordinates(), &[811589153, 2434767459, -1623178306]);
    }

    parameterized_test::create!{ example_moves, (init, idx, expected), {
        let mut encr = Encrypted::create(&init);
        println!("Original: {:?} --- {}", encr, encr);
        encr.mix_index(idx);
        println!("Mix({}): {:?} --- {}", idx, encr, encr);
        assert_eq!(encr.make_vec(), &expected);
    } }
    example_moves! {
        example1: ([4, 5, 6, 1, 7, 8, 9], 3, [4, 5, 6, 7, 1, 8, 9]),
        example2: ([4, -2, 5, 6, 7, 8, 9], 1, [4, 5, 6, 7, 8, -2, 9]),
        // note all idx values are relative to the initial array, not the partially-mixed arrays used as init
        idx0: ([1, 2, -3, 3, -2, 0, 4], 0, [2, 1, -3, 3, -2, 0, 4]),
        idx1: ([2, 1, -3, 3, -2, 0, 4], 0, [1, -3, 2, 3, -2, 0, 4]),
        idx2: ([1, -3, 2, 3, -2, 0, 4], 1, [1, 2, 3, -2, -3, 0, 4]),
        idx3: ([1, 2, 3, -2, -3, 0, 4], 2, [1, 2, -2, -3, 0, 3, 4]),
        idx4: ([1, 2, -2, -3, 0, 3, 4], 2, [-2, 1, 2, -3, 0, 3, 4]), // equivalent to [1, 2, -3, 0, 3, 4, -2]
        idx5: ([1, 2, -3, 0, 3, 4, -2], 3, [1, 2, -3, 0, 3, 4, -2]),
        idx6: ([1, 2, -3, 0, 3, 4, -2], 5, [1, 2, -3, 4, 0, 3, -2]),
    }
}
