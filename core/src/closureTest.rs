use std::*;
fn by_default_arg_passing() {
    /* 
     * This is borrow
     * Or by ref
     */

    let name:String = String::from("String");
    let f = || -> () { println!("name:{}", name);};
    f();
    println!("name is not moved: {}", name);

}

fn move_closure() {
    let name:String = String::from("String");
    let f = move || -> () { println!("name:{}", name);};
    f();
    /* 
    ** This will give error **
    println!("name is not moved: {}", name);
    */
}

fn borrow_and_move_mixed() {
    let name:String = String::from("String");
    let city:String = String::from("Chicago");
    let f = |a:String, b:&String| -> () { println!("name:{}, city:{}", a, b);};
    f(name, &city);
    // println!("name:{}". name); // Error because moved!!!
    println!("city:{}", city); 

}


pub fn test() {
    by_default_arg_passing();
    move_closure();
    borrow_and_move_mixed();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _test() {
        test();
    }
}