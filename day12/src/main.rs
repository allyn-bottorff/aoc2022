use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

// 83 -> 'S' (starting point)
// 69 -> 'E' (ending point)

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    neighbors: Vec<usize>,
    coord: (usize, usize),
}
impl Node {
    fn new(x:usize, y:usize) -> Self {
        Self {
            coord: (x, y),
            neighbors: Vec::new(),
        }
    }
}

impl Graph {
    fn add_node(&mut self, x: usize, y: usize) {
        self.nodes.push(Node::new(x,y));
    }
    fn find(&self, coord: (usize, usize)) -> Option<usize> {
        self.nodes.iter().position(|x| x.coord == coord)
    }
    fn new() -> Graph {
        let graph: Graph = Graph { nodes: Vec::new() };

        graph
    }
}

fn read_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    lines
}

fn make_topo(lines: Lines<BufReader<File>>) -> Vec<Vec<u8>> {
    let mut topo: Vec<Vec<u8>> = Vec::new();

    for line_r in lines {
        let line = line_r.unwrap();

        let heights = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
        topo.push(heights);
    }

    topo
}

fn make_graph(topo: Vec<Vec<u8>>) -> Graph {
    let mut graph: Graph = Graph::new();
    for y in 0..topo.len() {
        for x in 0..topo[y].len() {
            graph.add_node(x, y);
        }
    }

    for y in 0..topo.len() {
        for x in 0..topo[y].len() {
            let val = topo[y][x];

            //left
            if x >= 1{
                if topo[y][x-1] as i32 - val as i32 <= 1 {
                    let index = graph.find((x,y)).unwrap();
                    let neighbor = graph.find((x-1,y)).unwrap();
                    graph.nodes[index].neighbors.push(neighbor);
                }
            }
            //right
            if x < topo[y].len() - 1{
                if topo[y][x+1] as i32 - val as i32 <= 1 {
                    let index = graph.find((x,y)).unwrap();
                    let neighbor = graph.find((x+1,y)).unwrap();
                    graph.nodes[index].neighbors.push(neighbor);
                }
            }
            //down
            if y >= 1{
                if topo[y][y-1] as i32 - val as i32 <= 1 {
                    let index = graph.find((x,y)).unwrap();
                    let neighbor = graph.find((x,y-1)).unwrap();
                    graph.nodes[index].neighbors.push(neighbor);
                }
            }
            //up
            if y < topo.len() - 1{
                if topo[y+1][x] as i32 - val as i32 <= 1 {
                    let index = graph.find((x,y)).unwrap();
                    let neighbor = graph.find((x,y+1)).unwrap();
                    graph.nodes[index].neighbors.push(neighbor);
                }
            }
        }
    }

    graph
}

fn main() {
    println!("Hello, world!");

    let lines = read_file("tests/day12.txt");

    let topo = make_topo(lines);

    for line in &topo {
        println!("{:?}", line);
    }

    let graph = make_graph(topo);

    println!("{:#?}", graph);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_the_tests() {}

    #[test]
    fn find_a_node() {
        let mut graph = Graph::new();
        graph.add_node(1, 1);
        graph.add_node(2, 2);

        println!("{}",graph.nodes.len());


        assert_eq!( graph.nodes.iter().position(|x| x.coord == (1,1)), Some(0));
        assert_eq!( graph.nodes.iter().position(|x| x.coord == (2,2)), Some(1));
        
    }
}
