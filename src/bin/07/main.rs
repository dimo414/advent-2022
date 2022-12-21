use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use anyhow::{anyhow, bail, ensure, Error, Result};

fn main() -> Result<()> {
    let executions = parse_input(include_str!("input.txt"))?;

    let filesystem = construct_filesystem(&executions)?;
    let dirs = directory_sizes(&filesystem);

    let small_directories_size: u32 = dirs.values().filter(|&&s| s <= 100000).sum();
    println!("Small directories: {}", small_directories_size);

    let needed = cleanup_directory(&dirs)?;
    println!("Must free: {}", needed);

    Ok(())
}

// Can be removed if https://github.com/rust-lang/rfcs/issues/2208 is fixed
fn resolve_path<P: AsRef<Path>>(root: &Path, dir: P) -> Result<PathBuf> {
    let dir = dir.as_ref();
    if dir == Path::new("..") {
        Ok(root.parent().ok_or_else(|| anyhow!("Cannot resolve parent of /"))?.to_path_buf())
    } else {
        Ok(root.join(dir))
    }
}

fn construct_filesystem(executions: &[Execution]) -> Result<BTreeMap<PathBuf, u32>> {
    let mut filesystem = BTreeMap::new();
    let mut cur_dir = None;
    for execution in executions {
        match &execution.command {
            Command::Cd(dir) => {
                ensure!(execution.output.is_empty(), "{:?}", execution.output);
                if dir.starts_with('/') {
                    cur_dir = Some(Path::new(dir).to_path_buf());
                } else {
                    cur_dir = Some(resolve_path(&cur_dir.ok_or_else(|| anyhow!("Unknown cwd"))?,Path::new(dir))?);
                }
            },
            Command::Ls => {
                let cur_dir = cur_dir.as_ref().ok_or_else(|| anyhow!("Unknown cwd"))?;
                for line in &execution.output {
                    let parts: Vec<_> = line.split(' ').collect();
                    ensure!(parts.len() == 2);
                    let path = cur_dir.join(parts[1]);
                    if parts[0] == "dir" {
                        // no need to record directories, just check that it doesn't collide
                        ensure!(!filesystem.contains_key(&path));
                    } else {
                        let size = parts[0].parse()?;
                        if let Some(prior) = filesystem.insert(path, size) {
                            ensure!(prior == size, "Filesystem has changed!");
                        }
                    }
                }
            },
        };
    }
    Ok(filesystem)
}

fn directory_sizes(files: &BTreeMap<PathBuf, u32>) -> BTreeMap<PathBuf, u32> {
    let mut dirs = BTreeMap::new();
    for (path, size) in files {
        let mut path = path.parent();
        while let Some(dir) = path {
            dirs.entry(dir.to_path_buf()).and_modify(|s| *s += size).or_insert(*size);
            path = dir.parent();

        }
    }
    dirs
}

fn cleanup_directory(dirs: &BTreeMap<PathBuf, u32>) -> Result<u32> {
    let disk_size = 70000000;
    let need_free = 30000000;
    let used = dirs.get(Path::new("/")).cloned().ok_or_else(|| anyhow!("No root"))?;
    let free = disk_size - used;
    let needed = need_free - free;
    dirs.values().filter(|&&s| s >= needed).cloned().min().ok_or_else(|| anyhow!("No min"))
}

#[derive(Debug)]
struct Execution {
    command: Command,
    output: Vec<String>,
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(command: &str) -> Result<Self> {
        let parts: Vec<_> = command.split(' ').collect();
        ensure!(!parts.is_empty());
        match parts[0] {
            "ls" => {
                ensure!(parts.len() == 1);
                Ok(Command::Ls)
            },
            "cd" => {
                ensure!(parts.len() == 2);
                Ok(Command::Cd(parts[1].to_string()))
            }
            _ => bail!("Unknown command"),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Execution>> {
    let mut commands = Vec::new();
    let mut lines = input.lines().peekable();
    while let Some(command) = lines.next() {
        ensure!(command.starts_with("$ "), "Not a command!");
        let command = command[2..].parse()?;
        let mut output = Vec::new();
        while let Some(line) = lines.peek() {
            if line.starts_with("$ ") { break; }
            output.push(lines.next().expect("Already peeked").to_string());
        }
        commands.push(Execution{command, output});
    }
    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebuild_fs() {
        let execution = parse_input(include_str!("example.txt")).unwrap();
        let filesystem = construct_filesystem(&execution).unwrap();
        assert_eq!(filesystem, [
            ("/a/e/i",584),
            ("/a/f", 29116),
            ("/a/g", 2557),
            ("/a/h.lst", 62596),
            ("/b.txt", 14848514),
            ("/c.dat", 8504156),
            ("/d/j", 4060174),
            ("/d/d.log", 8033020),
            ("/d/d.ext", 5626152),
            ("/d/k", 7214296)
        ].into_iter().map(|(p, s)| (Path::new(p).to_path_buf(), s)).collect());
    }

    #[test]
    fn dir_sizes() {
        let execution = parse_input(include_str!("example.txt")).unwrap();
        let filesystem = construct_filesystem(&execution).unwrap();
        let dirs = directory_sizes(&filesystem);
        assert_eq!(dirs, [
            ("/", 48381165),
            ("/a", 94853),
            ("/a/e", 584),
            ("/d", 24933642),
        ].into_iter().map(|(p, s)| (Path::new(p).to_path_buf(), s)).collect());

        assert_eq!(cleanup_directory(&dirs).unwrap(), 24933642);
    }
}
