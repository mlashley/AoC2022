use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Tree stuff based on
/// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn node(&mut self, val: T) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
}

// End borrowed code.

#[derive(Debug, Default)]
struct FilesystemThing {
    size: usize,
    name: String,
    is_dir: bool,
}

impl FilesystemThing {
    fn new(size: usize, name: String, is_dir: bool) -> Self {
        Self { size, name, is_dir }
    }
    fn newdir(name: String) -> Self {
        Self::new(usize::MAX, name, true)
    }
    fn newfile(name: String, size: usize) -> Self {
        Self::new(size, name, false)
    }
}

impl PartialEq for FilesystemThing {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.name == other.name && self.is_dir == other.is_dir
    }
}

fn main() {
    // Test
    let mut tree: ArenaTree<FilesystemThing> = parse("./input_sample.txt".into());
    let size = size_fstree(&mut tree, 0);
    debug_assert!(48381165 == size);
    let part1size = sum_less_than(&mut tree, 0);
    debug_assert!(95437 == part1size);
    // print_fstree(&tree, 0, "".into());
    debug_assert!(find_minimal_remover(&mut tree) == 24933642);

    // Part1
    let mut tree: ArenaTree<FilesystemThing> = parse("./input.txt".into());
    let size = size_fstree(&mut tree, 0);
    // print_fstree(&tree, 0, "".into());
    println!("Part1 TotalSize = {}", size);
    let part1size = sum_less_than(&mut tree, 0);
    println!("Part1 <100000 Size = {}", part1size);
    debug_assert!(1325919 == part1size); // Keep Part 1 solving :)

    // Part2
    println!(
        "Part2 - Delete directory with size {}",
        find_minimal_remover(&mut tree)
    );
}

fn parse(input_filename: String) -> ArenaTree<FilesystemThing> {
    let mut tree: ArenaTree<FilesystemThing> = ArenaTree::default();
    let top = tree.node(FilesystemThing::new(0, "/".into(), true));
    let mut current = top;

    if let Ok(lines) = read_lines(input_filename) {
        let cd_re = Regex::new(r"^\$ cd (.*)$").unwrap();
        let ls_re = Regex::new(r"^\$ ls").unwrap();
        let dir_re = Regex::new(r"^dir (.*)").unwrap();
        let file_re = Regex::new(r"^(\d+) (.*)").unwrap();

        for line in lines.flatten() {
            if let Some(caps) = cd_re.captures(&line) {
                let dirname = caps.get(1).unwrap().as_str();
                // println!("CD: {:?}",dirname);
                if "/".eq(dirname) {
                    current = top;
                } else if "..".eq(dirname) {
                    current = tree.arena[current].parent.unwrap();
                } else {
                    for child in &tree.arena[current].children {
                        // println!("Seeking child {} in {} => {}",dirname,child,tree.arena[*child].val.name);
                        if tree.arena[*child].val.name.eq(dirname) {
                            current = *child;
                            break;
                        }
                    }
                }
            }

            if ls_re.is_match(&line) {
                // println!("LS: ")
            }

            if let Some(caps) = dir_re.captures(&line) {
                let dirname = caps.get(1).unwrap().as_str();
                // println!("DIR: {}",dirname);
                let newdir = tree.node(FilesystemThing::newdir(dirname.into()));
                tree.arena[current].children.push(newdir);
                tree.arena[newdir].parent = Some(current);
            }

            if let Some(caps) = file_re.captures(&line) {
                let filename = caps.get(2).unwrap().as_str();
                let filesize = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                // println!("FILE: {} SIZE: {} ",filename,filesize);

                let newfile = tree.node(FilesystemThing::newfile(filename.into(), filesize));
                tree.arena[current].children.push(newfile);
                tree.arena[newfile].parent = Some(current);
            }
        }
    }
    tree
}

fn print_fstree(tree: &ArenaTree<FilesystemThing>, start_idx: usize, indent: String) {
    if indent.is_empty() {
        println!("ROOT:{}", tree.arena[start_idx].val.name);
    }
    for child in &tree.arena[start_idx].children {
        if tree.arena[*child].val.is_dir {
            println!(
                "{}{}@D<{}>",
                indent, tree.arena[*child].val.name, tree.arena[*child].val.size
            );
            let mut nextindent = indent.clone();
            nextindent.push_str("  ");
            print_fstree(tree, *child, nextindent);
        } else {
            println!(
                "{}{}<{}>",
                indent, tree.arena[*child].val.name, tree.arena[*child].val.size
            );
        }
    }
}

fn size_fstree(tree: &mut ArenaTree<FilesystemThing>, start_idx: usize) -> usize {
    let mut size: usize = 0;
    let children = tree.arena[start_idx].children.clone();
    for child in children {
        if tree.arena[child].val.is_dir {
            let s = size_fstree(tree, child);
            size += s;
        } else {
            size += tree.arena[child].val.size
        }
    }
    tree.arena[start_idx].val.size = size;
    size
}

fn sum_less_than(tree: &mut ArenaTree<FilesystemThing>, start_idx: usize) -> usize {
    let mut part1size: usize = 0;
    let children = tree.arena[start_idx].children.clone();
    for child in children {
        if tree.arena[child].val.is_dir {
            if tree.arena[child].val.size <= 100000 {
                // println!("Counting {} from {} (idx={})",tree.arena[child].val.size,tree.arena[child].val.name,child);
                part1size += tree.arena[child].val.size;
            }
            part1size += sum_less_than(tree, child);
        }
    }
    part1size
}

fn get_dir_sizes(tree: &mut ArenaTree<FilesystemThing>, start_idx: usize) -> Vec<usize> {
    let mut dir_sizes = Vec::new();
    let children = tree.arena[start_idx].children.clone();
    dir_sizes.push(tree.arena[start_idx].val.size);
    for child in children {
        if tree.arena[child].val.is_dir {
            dir_sizes.append(&mut get_dir_sizes(tree, child));
        }
    }
    dir_sizes
}

fn find_minimal_remover(tree: &mut ArenaTree<FilesystemThing>) -> usize {
    let size = size_fstree(tree, 0);
    let need_to_free = 30000000 - (70000000 - size);
    println!("Need to free {}", need_to_free);
    let mut ds = get_dir_sizes(tree, 0);
    ds.sort();
    for s in ds {
        if s > need_to_free {
            return s;
        }
    }
    0
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
