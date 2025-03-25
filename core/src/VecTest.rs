use std::mem;
use std::vec::Vec;

///
/// Vector is a growable, mutable, owned, heap allocated, contiguous collection of elements.
/// It is a linear data structure.
/// We are using std::vec::Vec
/// Time complexity is O(1) for insert, deletion at end.
/// 
pub fn test() {
    /* XXX: Vector */
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("Vector: {:?}", v);
    let val = match v.get(2) {
        Some(val) => val,
        None => &0,
    };
    println!("Value: {}", val);
    for i in &v {
        println!("Value: {}", i);
    }

    println!("Size of vector: {}", mem::size_of_val(&v));
    print!("Vector[0]: {}", v[0]);


    /* XXX: Vector of given size */
    let fruits = vec!["Apple", "Banana", "Cherry", "Date", "Fig"];
    // convert into VecDeque
    let fruits_VecDeque: std::collections::VecDeque<_> = fruits.into_iter().collect();
    println!("Fruits: {:?}", fruits_VecDeque);
    let val = match fruits_VecDeque.get(2) {
        Some(val) => val,
        None => &"Not found",
    };
    println!("Value: {}", val);

    // vector of hights
    let heights = vec![5.8, 6.0, 5.9, 5.10, 6.1];
    let hight_sum: f64 = heights.iter().sum();

    // more use of iter()
    // We can use iter().x where x can be:
    // sum, product, max, min, max_by, min_by, max_by_key, min_by_key, all, any, find, position, rposition, fold, reduce, collect, partition, partition_in_place, unzip, chain, cloned, copied, cycle, enumerate, filter, filter_map, flat_map, flatten, fuse, inspect, map, map_while, peekable, scan, skip, skip_while, step_by, take, take_while, zip


    


   
    

}
