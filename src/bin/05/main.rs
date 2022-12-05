use std::str::FromStr;
use std::time::Duration;
use anyhow::{ensure, Error, Result};

use advent_2022::parsing::*;
use advent_2022::terminal::{Color, Terminal, TerminalImage, TerminalRender};

fn main() -> Result<()> {
    let _drop = Terminal::init();
    let (mut stacks, instructions) = parse_input(include_str!("input.txt"))?;
    let mut stacks_pt2 = stacks.clone();

    Terminal::interactive_render(&VisualizeStacks::down(&stacks), Duration::from_millis(1000));
    for instr in &instructions {
        stacks.arrange_9000(instr);
    }
    Terminal::interactive_render(&VisualizeStacks::down(&stacks), Duration::from_millis(1000));
    println!("CrateMover 9000: {}", stacks.tops());
    Terminal::end_interactive();

    Terminal::interactive_render(&VisualizeStacks::down(&stacks_pt2), Duration::from_millis(1000));
    for instr in &instructions {
        stacks_pt2.arrange_9001(instr);
    }
    Terminal::interactive_render(&VisualizeStacks::down(&stacks_pt2), Duration::from_millis(1000));
    println!("CrateMover 9001: {}", stacks_pt2.tops());
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn arrange_9000(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.num {
            let cargo = self.stacks[instruction.from-1].pop().unwrap();
            Terminal::interactive_render(&VisualizeStacks::up(self,instruction.from-1, &[cargo]), Duration::from_millis(25));
            Terminal::interactive_render(&VisualizeStacks::up(self, instruction.to-1, &[cargo]), Duration::from_millis(25));
            self.stacks[instruction.to-1].push(cargo);
            Terminal::interactive_render(&VisualizeStacks::down(self), Duration::from_millis(50));
        }
    }

    fn arrange_9001(&mut self, instruction: &Instruction) {
        let from = &mut self.stacks[instruction.from-1];
        let lifted = from.split_off(from.len()-instruction.num);
        Terminal::interactive_render(&VisualizeStacks::up(self,instruction.from-1, &lifted), Duration::from_millis(100));
        Terminal::interactive_render(&VisualizeStacks::up(self,instruction.to-1, &lifted), Duration::from_millis(100));
        self.stacks[instruction.to-1].extend(lifted);
        Terminal::interactive_render(&VisualizeStacks::down(self), Duration::from_millis(150));
    }

    fn tops(&self) -> String {
        self.stacks.iter().map(|c| c.last().cloned().unwrap_or(' ')).collect()
    }
}

impl FromStr for Stacks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn get_row(line: &str) -> Vec<char> {
            let mut chars = line.chars();
            chars.next(); // skip col0
            chars.step_by(4).collect()
        }

        let mut lines: Vec<_> = s.lines().map(|s| s.to_string()).collect();
        ensure!(lines.pop().is_some());
        lines.reverse();
        let mut stacks = Vec::new();
        for row in lines.iter().map(|l| get_row(l)) {
            if stacks.len() < row.len() {
                stacks.resize_with(row.len(), Vec::new);
            }
            for (idx, col) in row.into_iter().enumerate().filter(|(_,c)| *c != ' ') {
                stacks[idx].push(col);
            }
        }
        Ok(Stacks{stacks})
    }
}

struct VisualizeStacks<'a> {
    stacks: &'a Stacks,
    lifted: Option<(usize, &'a [char])>,
}

impl<'a> VisualizeStacks<'a> {
    fn down(stacks: &'a Stacks) -> VisualizeStacks<'a> {
        VisualizeStacks{stacks, lifted:None}
    }

    fn up(stacks: &'a Stacks, col: usize, cargo: &'a [char]) -> VisualizeStacks<'a> {
        VisualizeStacks{stacks, lifted:Some((col, cargo))}
    }
}

impl<'a> TerminalRender for VisualizeStacks<'a> {
    fn render(&self, _w: usize, height_hint: usize) -> TerminalImage {
        static COLORS: &[Color] = &[
            Color::GREEN, Color::YELLOW, Color::RED, Color::BLUE, Color::MAGENTA, Color::CYAN, Color::GREY,
            Color::ORANGE, Color::C256(18), Color::C256(22), Color::C256(24), Color::C256(64), Color::C256(81),
            Color::C256(85), Color::C256(88), Color::C256(93), Color::C256(127), Color::C256(142), Color::C256(154), Color::C256(182),
            Color::C256(202), Color::C256(207), Color::C256(209), Color::C256(215), Color::C256(219), Color::C256(225),
        ];
        fn letter_color(c: char) -> Color {
            assert!(('A'..='Z').contains(&c));
            COLORS[c as usize - 'A' as usize]
        }

        let stack_height = self.stacks.stacks.iter().map(|s|s.len()).max().unwrap_or(5);
        let height = stack_height + 1 + self.lifted.map(|l| l.1.len()).unwrap_or(0);
        let width = self.stacks.stacks.len()+2;
        let mut image = vec![Color::BLACK; height*width];

        for (col, stack) in self.stacks.stacks.iter().enumerate() {
            for (row, cargo) in stack.iter().enumerate() {
                image[(height-row-1)*width+col+1] = letter_color(*cargo);
            }
        }

        if let Some((col, stack)) = self.lifted {
            for (row, cargo) in stack.iter().enumerate() {
                image[row*width+col+1] = letter_color(*cargo);
            }
        }
        // Make each crate three pixels wide
        let stretch = 3;
        let width = width*stretch;
        let mut pixels: Vec<_> = image.into_iter().flat_map(|c| std::iter::repeat(c).take(stretch)).collect();

        // Add a "floor" two rows tall
        pixels.extend(vec![Color::BROWN; width*2]);

        // Fill the top of the image up to the provided height, so the image stays pinned at the
        // bottom of the terminal window. This mostly works, but the image still jumps around a bit
        // when the height is taller than the height_hint. Would be nice to improve.
        if height_hint > height {
            let mut buffer = vec![Color::BLACK; (height_hint - height) * width];
            buffer.extend(pixels);
            pixels = buffer;
        }

        TerminalImage{ pixels, width, }
    }
}

struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = static_regex!(r"move (.*) from (.*) to (.*)");
        let caps = regex_captures(regex, s)?;
        let num = capture_group(&caps, 1).parse()?;
        let from = capture_group(&caps, 2).parse()?;
        let to = capture_group(&caps, 3).parse()?;
        Ok(Instruction{num, from, to})
    }
}

fn parse_input(input: &str) -> Result<(Stacks, Vec<Instruction>)> {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    ensure!(parts.len() == 2);

    let instr = parts[1].lines().map(|l| l.parse::<Instruction>()).collect::<Result<Vec<_>>>()?;
    Ok((parts[0].parse()?, instr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_mover_9000() -> Result<()> {
        let (mut stacks, instructions) = parse_input(include_str!("example.txt"))?;

        assert_eq!(instructions.len(), 4);
        stacks.arrange_9000(&instructions[0]);
        let expected: Stacks = include_str!("example-1.1.txt").parse()?;
        assert_eq!(stacks, expected);
        stacks.arrange_9000(&instructions[1]);
        let expected: Stacks = include_str!("example-1.2.txt").parse()?;
        assert_eq!(stacks, expected);
        stacks.arrange_9000(&instructions[2]);
        let expected: Stacks = include_str!("example-1.3.txt").parse()?;
        assert_eq!(stacks, expected);
        stacks.arrange_9000(&instructions[3]);
        let expected: Stacks = include_str!("example-1.4.txt").parse()?;
        assert_eq!(stacks, expected);

        assert_eq!(stacks.tops(), "CMZ");
        Ok(())
    }

    #[test]
    fn crate_mover_9001() -> Result<()> {
        let (mut stacks, instructions) = parse_input(include_str!("example.txt"))?;

        assert_eq!(instructions.len(), 4);
        stacks.arrange_9001(&instructions[0]);
        let expected: Stacks = include_str!("example-2.1.txt").parse()?;
        assert_eq!(stacks, expected);
        stacks.arrange_9001(&instructions[1]);
        let expected: Stacks = include_str!("example-2.2.txt").parse()?;
        assert_eq!(stacks, expected);
        stacks.arrange_9001(&instructions[2]);
        let expected: Stacks = include_str!("example-2.3.txt").parse()?;
        assert_eq!(stacks, expected);
        stacks.arrange_9001(&instructions[3]);
        let expected: Stacks = include_str!("example-2.4.txt").parse()?;
        assert_eq!(stacks, expected);

        assert_eq!(stacks.tops(), "MCD");
        Ok(())
    }
}
