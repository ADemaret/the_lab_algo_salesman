use std::collections::HashMap;

///
/// link between real data and id's
///
pub struct Data {
    pub graph: HashMap<(usize, usize), usize>,
    nodes: Vec<String>,
    index: HashMap<String, usize>,
}

impl Data {
    ///
    /// input file parsing
    /// updates index & graph
    ///
    pub fn new(input: &str) -> Data {
        let mut graph = HashMap::new();
        let mut nodes = Vec::new();
        // on a des villes définies par leur index
        let mut index = HashMap::new();

        input.lines().enumerate().for_each(|(i, line)| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            // first line of input file is the list of node names
            if i == 0 {
                for (v, nom) in parts.iter().enumerate() {
                    index.insert(nom.to_string(), v);
                    nodes.push(nom.to_string())
                }
            } else {
                for (d, p) in parts.iter().enumerate() {
                    if d != 0 {
                        graph.insert((i - 1, d - 1), p.parse().unwrap());
                    }
                }
            }
        });
        // println!("{:?}", index);
        // println!("{:?}", graph);

        Data {
            graph,
            nodes,
            index,
        }
    }

    pub fn get_node_id(&self, node: String) -> usize {
        *(self.index.get(&node).unwrap())
    }

    pub fn get_starting_other(&self, start_node: usize) -> Vec<usize> {
        let mut other = Vec::new();
        for i in &self.index {
            if *i.1 != start_node {
                other.push(*i.1);
            }
        }
        other
    }

    pub fn display_path(&self, path: Vec<usize>) {
        println!("Complete path :");
        let mut dist = 0;
        for v in 0..(path.len() - 1) {
            dist += self.graph.get(&(path[v], path[v + 1])).unwrap();
            println!(
                "  from {} ({}) to {} ({}) : {} km",
                self.nodes[path[v]],
                path[v],
                self.nodes[path[v + 1]],
                path[v + 1],
                dist
            )
        }
    }
}
