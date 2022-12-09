use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
        for node in &self.arena {
            if node.val == val {
                return node.index;
            }
        }

        let index = self.arena.len();
        self.arena.push(Node::new(index, val));
        index
    }

    fn get_children<'a>(
        &'a self,
        index: usize,
        mut child_list: &'a mut Vec<usize>,
    ) -> &mut Vec<usize> {
        child_list.extend_from_slice(&self.arena[index].children);
        for child in &self.arena[index].children {
            child_list = self.get_children(*child, child_list);
        }

        child_list
    }

    // fn size(&self) -> usize {
    //     self.arena.len()
    // }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    index: usize,
    val: T,
    files: Vec<i32>,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(index: usize, val: T) -> Self {
        Self {
            index,
            files: vec![],
            val,
            parent: None,
            children: vec![],
        }
    }
}

enum LineType {
    CD(String),
    LS,
    File((i32, String)),
    Dir(String),
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_file(file: &str) -> ArenaTree<String> {
    let lines = read_lines(file).unwrap();
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let root = tree.node("/".into());

    let mut current_dir: usize = root;
    for line_result in lines {
        let line = line_result.unwrap();
        let line_type = parse_line(line);
        match line_type {
            LineType::CD(dir_name) => {
                if dir_name == ".." {
                    current_dir = match tree.arena[current_dir].parent {
                        Some(dir) => dir,
                        None => root,
                    }
                } else {
                    current_dir = tree.node(dir_name);
                }
            }
            LineType::LS => {
                continue;
            }
            LineType::Dir(dir_name) => {
                let found_dir = tree.node(dir_name);
                tree.arena[found_dir].parent = Some(current_dir);
                tree.arena[current_dir].children.push(found_dir);
            }
            LineType::File((size, _name)) => {
                tree.arena[current_dir].files.push(size);
            }
        }
    }
    tree
}

fn get_recursive_dir_size(tree: &ArenaTree<String>, directory: usize) -> i32 {
    let mut total_size: i32 = 0;
    let mut children: Vec<usize> = Vec::new();

    for file in &tree.arena[directory].files {
        total_size += *file;
    }
    children = tree.get_children(directory, &mut children).to_vec();
    for child in children {
        for file in &tree.arena[child].files {
            total_size += *file;
        }
    }

    total_size
}

fn get_total_size(tree: &ArenaTree<String>) -> i32 {
    let mut total_size: i32 = 0;

    for node in &tree.arena {
        for file_size in &node.files {
            total_size += file_size;
        }
    }
    total_size
}

/// Interpret the line type and package relevent info.
fn parse_line(line: String) -> LineType {
    let line_parts: Vec<&str> = line.split(' ').collect();
    if line_parts[0] == "$" && line_parts[1] == "cd" {
        return LineType::CD(line_parts[2].to_string());
    }
    if line_parts[0] == "$" && line_parts[1] == "ls" {
        return LineType::LS;
    }
    if line_parts[0] == "dir" {
        return LineType::Dir(line_parts[1].to_string());
    } else {
        let size: i32 = line_parts[0].parse::<i32>().unwrap();
        return LineType::File((size, line_parts[1].to_string()));
    }
}

fn main() {
    println!("Hello, world!");

    let fs_tree = process_file("./day7.txt");

    println!("{}", get_total_size(&fs_tree));
    println!("{}", get_recursive_dir_size(&fs_tree, 0));

    // for dir in &fs_tree.arena {
    //     let dir_size = get_recursive_dir_size(&fs_tree, dir.index);
    //     println!("Directory name: {:?} size: {:?}", dir.val, dir_size)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_root_size() {
        let fs_tree = process_file("./tests/day7.txt");
        let total_size = get_total_size(&fs_tree);
        assert_eq!(total_size, 48381165)
    }

    #[test]
    fn test_get_recursive_sizze() {
        let fs_tree = process_file("./tests/day7.txt");
        let total_size = get_recursive_dir_size(&fs_tree, 0);
        assert_eq!(total_size, 48381165)
    }

    #[test]
    fn test_get_tree_children() {
        let fs_tree = process_file("./tests/day7.txt");
        let mut children: Vec<usize> = Vec::new();
        children = fs_tree.get_children(0, &mut children).to_vec();

        println!("{:?}", children);

        assert_eq!(children, vec![1, 2, 3])
    }

    // #[test]
    // fn test_recursive_dir_size() {
    //     let fs_tree = process_file("./tests/day7.txt");
    //     let total_size = fs_tree.get_children
    //     assert_eq!(total_size, 48381165)
    // }
}
