use std::collections::HashMap;
use std::collections::LinkedList;
use std::vec::Vec;

///
/// Graph is a collection of nodes and edges.
/// It is a non-linear data structure.
/// We can represent graph in multiple ways:
/// - HashMap
/// - Vec
/// - Vec of Vec
/// - Vec of LinkedList
/// 
pub fn test() {
    /* XXX: Graph */
     /* XXX: Graph of given size */
     let mut graph = HashMap::new();
     graph.insert(1, vec![2, 3]);
     graph.insert(2, vec![1, 3]);
     graph.insert(3, vec![1, 2]);
     for (node, neighbors) in &graph {
         println!("Node: {}, Neighbors: {:?}", node, neighbors);
     }
     let neighbors = match graph.get(&1) {
         Some(neighbors) => neighbors,
         None => &vec![],
     };
     println!("Neighbors: {:?}", neighbors);

    /* XXX: Graph */
    let mut graph = Vec::new();
    graph.push(vec![1, 2, 3]);
    graph.push(vec![0, 2, 3]);
    graph.push(vec![0, 1, 3]);
    graph.push(vec![0, 1, 2]);
    for i in &graph {
        println!("Value: {:?}", i);
    }

    /* XXX: Graph of given size */
    let size = 4;
    let mut graph = vec![vec![0; size]; size];
    graph[0][1] = 1;
    graph[0][2] = 1;
    graph[0][3] = 1;
    graph[1][0] = 1;
    graph[1][2] = 1;
    graph[1][3] = 1;
    graph[2][0] = 1;
    graph[2][1] = 1;
    graph[2][3] = 1;
    graph[3][0] = 1;
    graph[3][1] = 1;
    graph[3][2] = 1;
    for i in &graph {
        println!("Value: {:?}", i);
    }

    /* XXX: Graph of given size */
    let size = 4;
    let mut graph = vec![vec![]; size];
    graph[0].push(1);
    graph[0].push(2);
    graph[0].push(3);
    graph[1].push(0);
    graph[1].push(2);
    graph[1].push(3);

    for i in &graph {
        println!("Value: {:?}", i);
    }
    

    /* XXX: Graph of given size */
    let size = 4;
    let mut graph= vec![];
    for _ in 0..size {
        graph.push(std::collections::LinkedList::new());
    }
    graph[0].push_back(1);
    graph[0].push_back(2);
    graph[0].push_back(3);
    graph[1].push_back(0);
    graph[1].push_back(2);
    graph[1].push_back(3);
    for i in &graph {
        println!("Value: {:?}", i);
    }
}