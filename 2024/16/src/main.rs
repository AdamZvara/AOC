mod base;

use base::all_directions;
use base::Grid;
use base::Direction;
use base::Coord;

use std::collections::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

type Weight = u32;
type Node = (Coord, Direction);
type Edge = (Node, Weight);

struct Graph {
    nodes: Vec<Node>,
    edges: HashMap<Node, Vec<Edge>>,
}

// Get all intersections of paths in the grid
fn intersections(grid: &Grid<char>, seq: &Vec<Coord>) -> HashMap<Coord, Vec::<Direction>> {
    let mut result = HashMap::new();
    let mut neighbors = Vec::new();
    for (x, y) in seq {
        neighbors.clear();
        for dir in all_directions().iter() {
            let neigh = Direction::to_coord(dir);
            let neigh_coords = (x + neigh.0, y + neigh.1);
            if grid.at(&neigh_coords).unwrap() == '.' || grid.at(&neigh_coords).unwrap() == 'S' || grid.at(&neigh_coords).unwrap() == 'E' {
                neighbors.push(dir.clone());
            }
        }
        if neighbors.len() > 2 {
            result.insert((*x, *y), neighbors.clone());
        } else if neighbors.len() == 2 {
            if !(neighbors.contains(&Direction::Down) && neighbors.contains(&Direction::Up)) && !(neighbors.contains(&Direction::Left) && neighbors.contains(&Direction::Right)) {
                result.insert((*x, *y), neighbors.clone());
            }
        }
    }
    result
}

// Get all intersections connected by a straight line
fn connected_intersections(grid: &Grid<char>, intersections: &HashMap<Coord, Vec<Direction>>) -> HashMap<Node, Vec<Coord>> {
    let mut result = HashMap::new();
    for (coord, dirs) in intersections {
        for dir in dirs {
            let mut connected = Vec::new();
            let mut new_coord = *coord;
            loop {
                let neigh = Direction::to_coord(dir);
                new_coord = (new_coord.0 + neigh.0, new_coord.1 + neigh.1);
                if grid.at(&new_coord).unwrap() == '.' || grid.at(&new_coord).unwrap() == 'S' || grid.at(&new_coord).unwrap() == 'E' {
                    if intersections.contains_key(&new_coord) {
                        connected.push(new_coord);
                    }
                } else {
                    break;
                }
            }
            result.insert((*coord, dir.clone()), connected);
        }
    }
    result
}

// Dijkstra's algorithm
fn dijkstra(graph: &Graph, start: &Node) -> (HashMap<Node, Weight>, HashMap<Node, Vec<Node>>) {
    let mut dist = HashMap::new();
    let mut pq = PriorityQueue::new();
    let mut parents = HashMap::new();
    pq.push(start, Reverse(0));
    dist.insert(start.clone(), 0);

    while let Some(((node, dir), weight)) = pq.pop() {
        if let Some(edges) = graph.edges.get(&(*node, dir.clone())) {
            for (next_node, next_weight) in edges {
                let new_weight = weight.0 + next_weight;
                if !dist.contains_key(next_node) {
                    dist.insert(next_node.clone(), new_weight);
                    pq.push(next_node, Reverse(new_weight));
                    parents.insert(next_node.clone(), vec![(*node, dir.clone())]);
                } else if new_weight <= *dist.get(next_node).unwrap() {
                    if new_weight < *dist.get(next_node).unwrap() {
                        dist.insert(next_node.clone(), new_weight);
                        pq.push(next_node, Reverse(new_weight));
                        parents.insert(next_node.clone(), vec![(*node, dir.clone())]);
                    } else {
                        dist.insert(next_node.clone(), new_weight);
                        pq.push(next_node, Reverse(new_weight));
                        parents.get_mut(next_node).unwrap().push((*node, dir.clone()));
                    }
                }
            }
        }
    }
    (dist, parents)
}

// Get distance between two nodes `parent` and `current` and add all the coordinates in between to `visited`
fn visited_two_coords(parent: &Node, current: &Node, visited: &mut Vec<Coord>) {
    let mut dir = (parent.0.0 - current.0.0, parent.0.1 - current.0.1);
    if dir == (0, 0) {
        if !visited.contains(&current.0) {
            visited.push(current.0);
        }
        return;
    }
    if dir.0 == 0 {
        if dir.1.is_positive() {
            dir = Direction::Right.to_coord();
        } else {
            dir = Direction::Left.to_coord();
        }
    } else if dir.1 == 0 {
        if dir.0.is_positive() {
            dir = Direction::Down.to_coord();
        } else {
            dir = Direction::Up.to_coord();
        }
    }
    let mut tmp = (current.0.0, current.0.1);
    loop {
        tmp = (tmp.0 + dir.0, tmp.1 + dir.1);
        if tmp == parent.0 {
            if !visited.contains(&tmp) {
                visited.push(tmp);
            }
            break;
        }
        if !visited.contains(&tmp) {
            visited.push(tmp);
        }
    }
}

// Get all visited coordinates between start and end (in all possible paths)
fn get_visited(parents: &HashMap<Node, Vec<Node>>, start: &Node, end: &Node, visited: &mut Vec<Coord>, multiparent_visited: &mut Vec<Coord>) {
    let mut current = end;

    while parents.get(&current).unwrap().len() == 1 {
        if current.0 == start.0 {
            return;
        }

        let parent = &parents.get(&current).unwrap()[0];
        visited_two_coords(parent, &current, visited);

        current = &parents.get(&current).unwrap()[0];

        if parents.get(&current).is_none() {
            return;
        }
    }

    if parents.get(&current).is_none() || current.0 == start.0 {
        return;
    }

    for parent in parents.get(&current).unwrap() {
        visited_two_coords(parent, &current, visited);
        if multiparent_visited.contains(&parent.0) {
            continue;
        }
        get_visited(parents, start, parent, visited, multiparent_visited);
        multiparent_visited.push(parent.0);
    }
   
}

fn main() {
    let grid = Grid::from_file_as_chars("input");
    let coridors = grid.find('.');
    let start = grid.find('S')[0];
    let end = grid.find('E')[0];

    // Get all intersections + start and end in all directions
    let mut intersections = intersections(&grid, &coridors);
    intersections.insert(start, all_directions());
    intersections.insert(end, all_directions());

    // Get all connected intersections
    let connected = connected_intersections(&grid, &intersections);

    // Create a graph
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: HashMap::new(),
    };

    /* Add nodes to the graph from intersection in ALL directions - this is because intersections hold outgoing directions, not incoming
       For example: ####
                    #..#
                    #.##
                    #..#
        The intersection at top left has 2 outgoing directions - Down and Right, but can be accessed from the bottom intersection by going Up, which would end up in
        Up direction in that intersection
    */
    for intersection in &intersections {
        for d in all_directions() {
            graph.nodes.push((*intersection.0, d.clone()));
        }
    }

    // Add rotation edges between the same intersection but different direction with cost 1000
    for intersection in &intersections {
        for d in all_directions() {
            for d2 in all_directions() {
                if d != d2 {
                    let cost = if Direction::is_opposite(&d, &d2) { 2000 } else { 1000 };
                    graph.edges.entry((*intersection.0, d.clone())).or_insert(Vec::new()).push(((*intersection.0, d2.clone()), cost));
                }
            }
        }
    }

    // Straight edges cost 1 between connected intersections
    for ((coord, direction), connected) in &connected {
        for c in connected {
            let distance = ((c.0 - coord.0).abs() + (c.1 - coord.1)).abs() as u32;
            if graph.nodes.contains(&(*c, direction.clone())) {
                graph.edges.entry((*coord, direction.clone())).or_insert(Vec::new()).push(((*c, direction.clone()), distance));
            }
        }
    }

    // Find the shortest path from start to end
    let start_node = graph.nodes.iter().find(|n| n.0 == start && n.1 == Direction::Right).unwrap();
    let (distances_from_start, parents) = dijkstra(&graph, start_node);
    let mut end_scores = Vec::new();
    for d in all_directions() {
        let end_node = graph.nodes.iter().find(|n| n.0 == end && n.1 == d).unwrap();
        if let Some(end_score) = distances_from_start.get(&end_node) {
            end_scores.push((end_node, *end_score));
        }
    }


    println!("Part 1: {:?}", end_scores.iter().map(|(_, s)| s).min().unwrap());
    let end_node = end_scores.iter().min_by_key(|(_, s)| s).unwrap().0;

    let mut visited_nodes = vec![end_node.0];
    let mut tmp = Vec::new();
    get_visited(&parents, start_node, end_node, &mut visited_nodes, &mut tmp);
    println!("Part 2: {:?}", visited_nodes.len());
}