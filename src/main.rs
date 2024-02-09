use std::time::Instant;
mod data;
mod tsp;
use crate::data::Data;
use crate::tsp::*;

fn main() {
    println!("-- The lab - Algo salesman --");

    let now = Instant::now();

    let input = include_str!("../assets/distances.txt");
    let data = Data::new(input);
    let start_node = "Bruxelles".to_string();
    let retour = true;
    let answer = get_answer(&data, start_node.clone(), retour);
    display_answer(&start_node, retour, answer);

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

fn display_answer(start_node: &String, retour: bool, answer: usize) {
    print!("To visit every node from {}", start_node);
    if retour {
        print!(" (and come back to start)");
    } else {
        print!(" (no coming back to start)");
    }
    println!(", it cost {} km", answer);
}

fn get_answer(data: &Data, start_node: String, retour: bool) -> usize {
    //
    let start_id = data.get_node_id(start_node.clone());

    // "other" : unvisited nodes
    let mut other = data.get_starting_other(start_id);

    // call to TSP
    let mut answer = tsp(&data.graph, start_id, start_id, &mut other, retour);
    straighten_path(&mut answer.1, start_id);

    //
    data.display_path(answer.1);

    answer.0
}

///
/// unit tests
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo() {
        let input = include_str!("../assets/distances.txt");
        let data = Data::new(input);
        let start_node = "Bruxelles".to_string();
        let retour = true;
        assert_eq!(get_answer(&data, start_node.clone(), retour), 546);
        let start_node = "Bruges".to_string();
        assert_eq!(get_answer(&data, start_node.clone(), retour), 546);
        let start_node = "Bruxelles".to_string();
        let retour = false;
        assert_eq!(get_answer(&data, start_node.clone(), retour), 468);
        let start_node = "Bruges".to_string();
        assert_eq!(get_answer(&data, start_node.clone(), retour), 410);
        let start_node = "Liège".to_string();
        assert_eq!(get_answer(&data, start_node.clone(), retour), 426); // because input file is not symetric
    }
}
