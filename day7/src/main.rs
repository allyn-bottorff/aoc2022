use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Default)]
struct ArenaTree {
    arena: Vec<Node>,
}
impl ArenaTree {
    fn node(&mut self, val: String) -> usize {
        for node in &self.arena {
            if node.val == val {
                return node.index;
            }
        }

        let index = self.arena.len();
        self.arena.push(Node::new(index, val));
        index
    }

    fn get_full_path_prefix(&self, starting_index: usize) -> String {
        let mut names: Vec<&str> = Vec::new();
        let mut node: usize = starting_index;
        let mut full_path: String = String::from("");
        loop {
            names.push(&self.arena[node].val);
            node = match &self.arena[node].parent {
                Some(node) => *node,
                None => break,
            };
        }
        for name in names {
            full_path = format!("{}{}", full_path, name);
        }
        full_path
    }

    fn get_children(&self, starting_index: usize) -> Vec<usize> {
        let mut all_children: Vec<usize> = Vec::new();
        let mut nodes: Vec<usize> = Vec::new();
        let mut first_node: bool = true;
        nodes.push(starting_index);

        loop {
            let node = match nodes.pop() {
                Some(node) => node,
                None => break,
            };

            for child in &self.arena[node].children {
                if all_children.contains(child) {
                    // println!("found_loop at index {}", child);
                    break;
                }
                nodes.push(*child)
            }
            if first_node == false {
                all_children.push(node)
            } else {
                first_node = false;
            }
        }

        all_children
    }

    // fn get_children_2<'a>(
    //     &'a self,
    //     index: usize,
    //     mut child_list: &'a mut Vec<usize>,
    // ) -> &mut Vec<usize> {
    //     child_list.extend_from_slice(&self.arena[index].children);
    //     for child in &self.arena[index].children {
    //         child_list = self.get_children_2(*child, child_list);
    //     }

    //     child_list
    // }

    // fn size(&self) -> usize {
    //     self.arena.len()
    // }
}

#[derive(Debug)]
struct Node {
    index: usize,
    val: String,
    files: Vec<i32>,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(index: usize, val: String) -> Self {
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

fn process_file(file: &str) -> ArenaTree {
    let lines = read_lines(file).unwrap();
    let mut tree: ArenaTree = ArenaTree::default();
    let root = tree.node("/".into());

    let mut current_dir: usize = root;
    for line_result in lines {
        let line = line_result.unwrap();
        let line_type = parse_line(line);
        let parent_dir_name = &tree.arena[current_dir].val.to_string();
        match line_type {
            LineType::CD(dir_name) => {
                if dir_name == ".." {
                    current_dir = match tree.arena[current_dir].parent {
                        Some(dir) => dir,
                        None => root,
                    }
                } else if dir_name == "/" {
                    current_dir = root;
                } else {
                    current_dir = tree.node(format!("{}{}/", parent_dir_name, dir_name));
                }
            }
            LineType::LS => {}
            LineType::Dir(dir_name) => {
                // let parent_dir_name = &tree.arena[current_dir].val.to_string();
                let found_dir = tree.node(format!("{}{}/", parent_dir_name, dir_name));
                // let found_dir = tree.node(dir_name);
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

fn get_recursive_dir_size(tree: &ArenaTree, directory: usize) -> i32 {
    let mut total_size: i32 = 0;
    // let mut children: Vec<usize> = Vec::new();

    for file in &tree.arena[directory].files {
        total_size += *file;
    }
    let children = tree.get_children(directory);
    for child in children {
        for file in &tree.arena[child].files {
            total_size += *file;
        }
    }

    total_size
}

fn get_total_size(tree: &ArenaTree) -> i32 {
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

    // let fs_tree = process_file("./tests/day7.txt");
    // println!("{:?}", fs_tree);

    let fs_tree = process_file("./day7.txt");

    let children = fs_tree.get_children(0);

    let full_path = fs_tree.get_full_path_prefix(10);
    println!("full path from node index 10: {}", full_path);

    println!("total child dirs: {}", children.len());
    let total_size: i32 = get_total_size(&fs_tree);
    println!("total size directly: {}", total_size);
    println!(
        "total size recursively: {}",
        get_recursive_dir_size(&fs_tree, 0)
    );

    let mut at_most_100k_dirs: Vec<i32> = Vec::new();
    let mut frees_enough_space_dirs: Vec<i32> = Vec::new();

    let required_space_to_free: i32 = 70000000 - get_total_size(&fs_tree);

    for dir in &fs_tree.arena {
        let dir_size = get_recursive_dir_size(&fs_tree, dir.index);
        if dir_size < 100000 {
            at_most_100k_dirs.push(dir_size);
        }
        if dir_size > required_space_to_free {
            frees_enough_space_dirs.push(dir_size);
        }

        println!("Directory name: {:?} size: {:?}", dir.val, dir_size)
    }

    let mut big_dir_sum: i32 = 0;
    for dir in at_most_100k_dirs {
        big_dir_sum += dir;
    }

    println!("required space to free: {}", required_space_to_free);
    println!("sum of big dirs: {}", big_dir_sum);

    frees_enough_space_dirs.sort();
    println!("acceptable_directories: {:?}", frees_enough_space_dirs);
    println!(
        "smallest acceptable directory: {}",
        frees_enough_space_dirs[0]
    );
    let smallest = frees_enough_space_dirs.pop().unwrap();
    println!("largest acceptable directory: {}", smallest);
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
    fn test_get_recursive_size() {
        let fs_tree = process_file("./tests/day7.txt");
        let total_size = get_recursive_dir_size(&fs_tree, 0);
        assert_eq!(total_size, 48381165)
    }

    #[test]
    fn test_get_tree_children() {
        let fs_tree = process_file("./tests/day7.txt");
        let mut children: Vec<usize> = fs_tree.get_children(0);

        println!("{:?}", children);

        assert_eq!(children.sort(), vec![1, 2, 3].sort())
    }

    #[test]
    fn test_full_path() {
        let fs_tree = process_file("./tests/day7.txt");
        let full_path = fs_tree.get_full_path_prefix(3);
        println!("full path? {}", full_path);
    }

    // #[test]
    // fn test_recursive_dir_size() {
    //     let fs_tree = process_file("./tests/day7.txt");
    //     let total_size = fs_tree.get_children
    //     assert_eq!(total_size, 48381165)
    // }
}
