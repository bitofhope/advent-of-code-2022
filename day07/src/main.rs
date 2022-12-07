use std::collections::BTreeMap;
use std::env;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    subdirs: Vec<usize>,
    files: Vec<usize>,
    parent: usize
}
impl Directory {
    fn new(name: String, parent: usize) -> Self {
        Directory {
            name: name,
            subdirs: Vec::new(),
            files: Vec::new(),
            parent: parent,
        }
    }
}

#[derive(Debug, Clone)]
struct File {
    _name: String,
    size: usize,
}

#[derive(Debug, Clone)]
enum InputLine {
    CdUp,
    Cd { directory: String },
    Ls,
    Directory { name: String },
    File { size: usize, name: String },
}
impl From<String> for InputLine {
    fn from(s: String) -> InputLine {
        let mut words = s.split(' ');
        match (words.next(), words.next(), words.next()) {
            (Some("$"),   Some("cd"), Some("..")) => Self::CdUp,
            (Some("$"),   Some("cd"), Some(d))    => Self::Cd { directory: d.to_string() },
            (Some("$"),   Some("ls"), None)       => Self::Ls,
            (Some("dir"), Some(d),    None)       => Self::Directory { name: d.to_string() },
            (Some(size),  Some(name), None)       => Self::File {
                size: size.parse().unwrap(),
                name: name.to_string()
            },
            _ => panic!("Unrecognized input: {s}"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = std::fs::File::open(file_path).unwrap();
    let mut lines = std::io::BufReader::new(file).lines();

    let mut ids = 0..;
    let root_id = ids.next().unwrap();
    let root: Directory;
    if let InputLine::Cd { directory: d } = lines.next().unwrap().unwrap().into() {
        root = Directory::new(d, root_id);
    } else {
        panic!("Expected '$ cd /'");
    }
    let mut pwd = root_id;
    let mut files = BTreeMap::new();
    let mut dirs = BTreeMap::new();
    dirs.insert(root_id, root);

    while let Some(Ok(line)) = lines.next() {
        match InputLine::from(line) {
            InputLine::CdUp => {
                pwd = dirs.get(&pwd).unwrap().parent;
            },
            InputLine::Cd { directory: d } => {
                pwd = *(dirs.get(&pwd)
                    .unwrap()
                    .subdirs
                    .iter()
                    .find(|id| dirs.get(id).unwrap().name == d)
                    .unwrap());
            },
            InputLine::Ls => { },
            InputLine::Directory { name: d } => {
                let id = ids.next().unwrap();
                dirs.insert(id, Directory::new(d, pwd));
                dirs.get_mut(&pwd).unwrap().subdirs.push(id);
            },
            InputLine::File { size: s, name: n } => {
                let id = ids.next().unwrap();
                files.insert(id, File { _name: n, size: s });
                dirs.get_mut(&pwd).unwrap().files.push(id);
            },
        }
    }
    part1(&dirs, &files);
    part2(&dirs, &files);
}

fn size_of_dir(id: &usize, ds: &BTreeMap<usize, Directory>,
    fs: &BTreeMap<usize, File>) -> usize
{
    let dir = ds.get(id).unwrap();
    let mut total = 0;
    for f in dir.files.iter() {
        total += fs.get(f).unwrap().size;
    }
    for d in dir.subdirs.iter() {
        total += size_of_dir(d, &ds, &fs);
    }
    total
}

fn part1(dirs: &BTreeMap<usize, Directory>, files: &BTreeMap<usize, File>) {
    let mut total = 0;
    for d in dirs.keys() {
        let size = size_of_dir(d, dirs, files);
        if size <= 100_000 {
            total += size;
        }
    }
    println!("{total}");
}

fn part2(dirs: &BTreeMap<usize, Directory>, files: &BTreeMap<usize, File>) {
    let disk_size: usize = 70_000_000;
    let used: usize = size_of_dir(&0, dirs, files);
    let free = disk_size - used;
    let mut smallest = used;
    for d in dirs.keys() {
        let size = size_of_dir(d, dirs, files);
        if (size + free >= 30_000_000) && (size < smallest) {
            smallest = size;
        }
    }
    println!("{smallest}");
}

