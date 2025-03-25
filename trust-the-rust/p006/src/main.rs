use std::vec;

fn print_vec(arr : & Vec<i32>) ->() {
    let size = arr.len();
    print!("Array: ");
    for index in 0..size {
        print!("{} ", arr.get(index).unwrap());
    }
    println!("");
}

fn main() {
    let mut arr:Vec<i32> = Vec::new();
    arr.push(10);
    arr.push(20);

    for i in 0..10 {
        match arr.get(i) {
            Some(x) => {
                println!("arr[{}]={}", i, x);
            }
            _ => {
                println!("{} : is out of index", i)
            }
        }
    }


    for i in 0..10 {
        match arr.pop() {
            Some(x) => {
                println!("{}th time, poped: {}", i, x);
            }
            _ => {
                println!("{}th time, empty!!", i);
            }
        }
    }


    for i in 0..10{
        arr.push(i%5);
    }


    print_vec(& arr);

    arr.sort();



}
