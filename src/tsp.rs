use std::collections::HashMap;

///
/// Traveling Salesperson Algorithm
/// or
/// Held–Karp algorithm
///
/// graph : HashMap<(node_id_1,node_id_2), distance)>
/// start_node : id
/// current_node : id (=start_node at first call)
/// other : vector of all other node id's at first call
/// back_to-start : true if traveler has to come back to start
///
/// -> (total_distance, vector of node_id from end to start without the start node)
/// use straighten-path to get the correct path
pub fn tsp(
    graph: &HashMap<(usize, usize), usize>,
    start_node: usize,
    current_node: usize,
    other: &mut Vec<usize>,
    back_to_start: bool,
) -> (usize, Vec<usize>) {
    //
    if other.is_empty() {
        if back_to_start {
            // if we come back to start_node,
            // we must add the distance between start_node and current_node
            return (
                *graph.get(&(current_node, start_node)).unwrap(),
                vec![start_node], // path
            );
        } else {
            (0, Vec::new())
        }
    } else {
        let mut dists = Vec::new();
        //
        let other_clone = other.clone();
        for node in other {
            // d1k = distance between start_node and K_node
            let d1k = graph.get(&(current_node, *node)).unwrap();
            // K is removed from "other"
            let mut other_minus_k = other_clone.clone();
            let k_index = other_minus_k.iter().position(|&x| x == *node).unwrap();
            other_minus_k.remove(k_index);
            // tsp of K
            let tsp_k = tsp(graph, start_node, *node, &mut other_minus_k, back_to_start);
            let mut path = tsp_k.1;
            path.push(*node);
            dists.push((d1k + tsp_k.0, path));
        }
        dists.iter().min_by_key(|x| x.0).unwrap().clone()
    }
}

pub fn straighten_path(path: &mut Vec<usize>, start_node: usize) {
    path.push(start_node);
    path.reverse();
}
