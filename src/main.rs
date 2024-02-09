use std::{collections::HashMap, time::Instant};

fn main() {
    println!("-- The lab - Algo salesman --");

    let now = Instant::now();

    let input = include_str!("../assets/distances.txt");
    let depart = "Bruxelles".to_string();
    let retour = true;
    display_answer(&depart, retour, get_answer(input, &depart, retour));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

fn display_answer(depart: &String, retour: bool, answer: usize) {
    print!("Pour passer par toutes les villes au départ de {}", depart);
    if retour {
        print!("(et y revenir)");
    } else {
        print!("(sans y revenir)");
    }
    println!(", il faut faire {} km", answer);
}

fn get_answer(input: &str, depart: &String, retour: bool) -> usize {
    // on a des villes définies par leur index
    let mut index: HashMap<String, usize> = HashMap::new();
    // on a les distances entre chaque villes
    let mut graph = HashMap::new();

    // on reprend les données
    parse_input_file(input, &mut graph, &mut index);

    // on part de "start_point"
    // let depart = "Bruxelles".to_string();
    let start_node = *(index.get(depart).unwrap());

    // l'algo nous donne la distance pour parcourir tous les points
    // en option : retour au point de départ

    // "other" contient tous les noeuds sauf le point de départ
    let mut other = Vec::new();
    for i in &index {
        if *i.1 != start_node {
            other.push(*i.1);
        }
    }
    tsp(&graph, start_node, start_node, &mut other, retour)
}

///
/// Traveling Salesperson Algorithm
/// or
/// Held–Karp algorithm
///
fn tsp(
    graph: &HashMap<(usize, usize), usize>,
    start_node: usize,
    current_node: usize,
    other: &mut Vec<usize>,
    retour: bool,
) -> usize {
    if other.is_empty() {
        if retour {
            // si on veut retourner au point de départ,
            // on doit ajouter la distance entre le départ et ce point
            return *graph.get(&(current_node, start_node)).unwrap();
        } else {
            0
        }
    } else {
        let mut dists = Vec::new();
        // pour tous les noeuds voisins
        let other_clone = other.clone();
        for node in other {
            // d1k = distance du départ au noeud k
            let d1k = graph.get(&(current_node, *node)).unwrap();
            let mut other_minus_k = other_clone.clone();
            let k_index = other_minus_k.iter().position(|&x| x == *node).unwrap();
            other_minus_k.remove(k_index);
            let tsp_k = tsp(graph, start_node, *node, &mut other_minus_k, retour);
            dists.push(d1k + tsp_k);
        }
        *dists.iter().min().unwrap()
    }
}

///
/// donne la distance entre deux villes
/// (test pour vérifier le parsing)
///
fn get_distance(
    start: &str,
    end: &str,
    index: &HashMap<String, usize>,
    graph: &HashMap<(usize, usize), usize>,
) -> usize {
    let id1 = index.get(start).unwrap();
    let id2 = index.get(end).unwrap();
    *graph.get(&(*id1, *id2)).unwrap()
}

///
/// parsing du fichier
/// met à jout index & graph
///
fn parse_input_file(
    input: &str,
    graph: &mut HashMap<(usize, usize), usize>,
    index: &mut HashMap<String, usize>,
) {
    input.lines().enumerate().for_each(|(i, line)| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        // la première ligne est la liste des villes
        if i == 0 {
            for (v, nom) in parts.iter().enumerate() {
                index.insert(nom.to_string(), v);
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
}

///
/// tests unitaires
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distances() {
        let input = include_str!("../assets/distances.txt");
        let mut index = HashMap::new();
        let mut graph = HashMap::new();
        parse_input_file(input, &mut graph, &mut index);
        assert_eq!(get_distance("Anvers", "Charleroi", &index, &graph), 106);
        assert_eq!(get_distance("Anvers", "Anvers", &index, &graph), 0);
    }

    #[test]
    fn test_algo() {
        assert_eq!(
            get_answer(
                include_str!("../assets/distances.txt"),
                &"Bruxelles".to_string(),
                true
            ),
            546
        );
    }
}
