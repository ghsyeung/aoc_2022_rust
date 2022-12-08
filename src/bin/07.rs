mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_till},
        character::complete::{alphanumeric1, digit1, multispace1, space1},
        combinator::{map, map_res},
        multi::many0,
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    use crate::model::{ChangeDirectory, Command, FileEntryInput, LineType};

    fn parse_cd_up(s: &str) -> IResult<&str, ChangeDirectory> {
        map(tag(".."), |_| ChangeDirectory::Up)(s)
    }

    fn parse_cd_root(s: &str) -> IResult<&str, ChangeDirectory> {
        map(tag("/"), |_| ChangeDirectory::Root)(s)
    }

    fn parse_cd_down(s: &str) -> IResult<&str, ChangeDirectory> {
        map(alphanumeric1, |dir: &str| {
            ChangeDirectory::Down(dir.to_owned())
        })(s)
    }

    fn parse_cd(s: &str) -> IResult<&str, Command> {
        map(
            preceded(tag("cd "), alt((parse_cd_up, parse_cd_root, parse_cd_down))),
            |cd| Command::Cd(cd),
        )(s)
    }

    fn parse_ls(s: &str) -> IResult<&str, Command> {
        map(tag("ls"), |_| Command::List)(s)
    }

    fn parse_command(s: &str) -> IResult<&str, Command> {
        preceded(tag("$ "), alt((parse_ls, parse_cd)))(s)
    }

    fn parse_file(s: &str) -> IResult<&str, FileEntryInput> {
        let parse_size = map_res(digit1, |s: &str| s.parse::<usize>());
        map(
            separated_pair(parse_size, space1, take_till(char::is_whitespace)),
            |(size, name): (usize, &str)| FileEntryInput::File((name.to_owned(), size)),
        )(s)
    }

    fn parse_dir(s: &str) -> IResult<&str, FileEntryInput> {
        map(
            preceded(tag("dir "), take_till(char::is_whitespace)),
            |name: &str| FileEntryInput::Dir(name.to_owned()),
        )(s)
    }

    fn parse_file_entry(s: &str) -> IResult<&str, FileEntryInput> {
        alt((parse_file, parse_dir))(s)
    }

    pub fn parse_line(s: &str) -> IResult<&str, LineType> {
        alt((
            map(parse_command, LineType::Command),
            map(parse_file_entry, LineType::FileEntry),
        ))(s)
    }

    pub fn parse_input(s: &str) -> IResult<&str, Vec<LineType>> {
        many0(terminated(parse_line, multispace1))(s)
    }
}

mod model {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum Command {
        Cd(ChangeDirectory),
        List,
    }

    #[derive(Debug)]
    pub enum ChangeDirectory {
        Up,
        Down(String),
        Root,
    }

    #[derive(Debug)]
    pub enum FileEntryInput {
        Dir(String),
        File((String, usize)),
    }

    #[derive(Debug)]
    pub enum LineType {
        Command(Command),
        FileEntry(FileEntryInput),
    }

    #[derive(Debug, Default)]
    pub struct Walk {
        pub dir_stack: Vec<String>,
    }

    impl Walk {
        pub fn path(&self) -> String {
            format!("/{}", self.dir_stack.join("/"))
        }

        pub fn all_parent_path(&self) -> Vec<String> {
            let mut s = "/".to_owned();
            let mut p: Vec<String> = vec!["/".to_owned()];
            for i in self.dir_stack.iter() {
                let c = format!("{}/{}", &s, &i);
                s = c.clone();
                p.push(c);
            }
            p
        }
    }

    #[derive(Debug)]
    pub enum FileEntry {
        Dir(HashMap<String, FileEntry>),
        File(usize),
    }

    #[derive(Default, Debug)]
    pub struct FileSystem {
        pub root: HashMap<String, FileEntry>,
    }

    #[derive(Debug)]
    pub struct FlatFileSystem(pub HashMap<String, usize>);
}

mod logic {
    use std::collections::HashMap;

    use crate::model::{Command, FileEntryInput, FlatFileSystem, LineType, Walk};

    pub fn gen_flat_file_system(lines: &[LineType]) -> color_eyre::Result<FlatFileSystem> {
        let mut fs = FlatFileSystem(HashMap::default());
        let mut walk = Walk::default();
        for line in lines {
            match line {
                LineType::Command(Command::Cd(cd)) => match cd {
                    crate::model::ChangeDirectory::Up => {
                        walk.dir_stack.pop();
                    }
                    crate::model::ChangeDirectory::Down(dir) => {
                        walk.dir_stack.push(dir.to_owned());
                    }
                    crate::model::ChangeDirectory::Root => {
                        walk.dir_stack.clear();
                    }
                },
                LineType::Command(Command::List) => {}
                LineType::FileEntry(FileEntryInput::Dir(_)) => {}
                LineType::FileEntry(FileEntryInput::File(f)) => {
                    // Since it's a file, we can just add the size to the current path
                    for p in walk.all_parent_path().iter() {
                        if let Some(s) = fs.0.get_mut(p) {
                            *s += f.1;
                        } else {
                            fs.0.insert(p.to_owned(), f.1);
                        }
                    }
                }
            }
        }
        Ok(fs)
    }

    /*
    fn look_up_folder<'a>(
        fs: &'a mut FileSystem,
        path: &[String],
    ) -> eyre::Result<&'a mut HashMap<String, FileEntry>> {
        if path.is_empty() {
            Ok(&mut fs.root)
        } else {
            let mut cur = &fs.root;
            for i in path.iter() {
                if let Some(d) = cur.get(i) {
                    match d {
                        FileEntry::Dir(d) => cur = d,
                        FileEntry::File(_) => {
                            Err(eyre::eyre!("Lookup folder found file: {:?}", path))?;
                        }
                    }
                } else {
                    Err(eyre::eyre!("Cannot find folder: {:?}", path))?
                }
            }
            Ok(&mut cur)
        }
    }

    pub fn gen_file_tree(lines: &[LineType]) -> color_eyre::Result<FileSystem> {
        let mut fs = FileSystem::default();
        let mut walk = Walk::default();

        for line in lines {
            match line {
                LineType::Command(Command::Cd(cd)) => match cd {
                    crate::model::ChangeDirectory::Up => {
                        if walk.dir_stack.is_empty() {
                            Err(eyre::eyre!("Cannot cd up"))?;
                        } else {
                            walk.dir_stack.pop();
                        }
                    }
                    crate::model::ChangeDirectory::Down(dir) => {
                        walk.dir_stack.push(dir.to_owned());
                    }
                    crate::model::ChangeDirectory::Root => {
                        walk.dir_stack.clear();
                    }
                },
                LineType::Command(Command::List) => {}
                LineType::FileEntry(fe) => {
                    let cwd = look_up_folder(&mut fs, &walk.dir_stack)?;
                }
            }
        }
    }
    */
}

pub fn part_one(input: &str) -> color_eyre::Result<usize> {
    let (_, lines) = parser::parse_input(input).map_err(|e| eyre::eyre!(e.to_owned()))?;
    let fs = logic::gen_flat_file_system(&lines)?;
    dbg!(&fs);
    Ok(fs
        .0
        .iter()
        .filter(|&(_name, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum())
}

const DISK_SPACE: usize = 70000000;
const UPDATE: usize = 30000000;
pub fn part_two(input: &str) -> color_eyre::Result<usize> {
    let (_, lines) = parser::parse_input(input).map_err(|e| eyre::eyre!(e.to_owned()))?;
    let fs = logic::gen_flat_file_system(&lines)?;
    let root_size = fs.0.get("/").ok_or(eyre::eyre!("Root folder is missing"))?;
    let need: usize = UPDATE - (DISK_SPACE - root_size);

    dbg!(&fs);
    fs.0.iter()
        .filter(|&(_name, size)| *size >= need)
        .map(|(_, size)| size)
        .min()
        .cloned()
        .ok_or(eyre::eyre!("No folder is small enough"))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input).unwrap(), 95437);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input).unwrap(), 24933642);
    }
}
