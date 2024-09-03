
use std::mem;
use std::string::String;
use std::vec::Vec;

/*
1. slice are read only
2. slice are a reference to a contiguous sequence of elements in a collection
3. slice are a view into a collection
4. slice are a smart pointer
5. slice are a fat pointer
6. slice are a double word
7. slice are a pointer to the start of the slice and the length of the slice

*/
fn test_array_vs_slice() {
    let array = [1, 2, 3, 4, 5];
    let slice = &array[..];

    println!("Size of array: {}", mem::size_of_val(&array));
    println!("Size of slice: {}", mem::size_of_val(&slice));
    println!("Length of array: {}", array.len());
    println!("Length of slice: {}", slice.len());
    println!("First element of array: {}", array[0]);
    println!("First element of slice: {}", slice[0]);
    println!("Array: {:?}", array);
    println!("Slice: {:?}", slice);


    let chars = ['a', 'b', 'c', 'd', 'e'];
    let slice = &chars[1..4];
    println!("Slice: {:?}", slice);


    // Time complexity of slicing is O(1)
    let chars = "Hellpo, World!";
    let slice = &chars[1..4];
    println!("Slice: {:?}", slice);

    // Time complexity of accessing a character by index is O(1)
    let chars = "Hello, World!";
    let char = chars.chars().nth(1).unwrap();
    println!("Char: {}", char);


    // Time complexity of accessing a character by index is O(1)
    let chars = "Hello, World!";
    let char = chars.chars().nth(1).unwrap();
    println!("Char: {}", char);


    let char_array = ['a', 'b', 'c', 'd', 'e'];
    let char_slice = &char_array[1..4];
    println!("Char Slice: {:?}", char_slice);
    let char_at_index = char_array[1];
    println!("Char at index: {}", char_at_index);
}


fn test_array_slice() {
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4];
    println!("Array: {:?}", array);
    println!("Slice: {:?}", slice);
    println!("Length of array: {}", array.len());
    println!("Length of slice: {}", slice.len());
    println!("First element of array: {}", array[0]);
    println!("First element of slice: {}", slice[0]);
    println!("Last element of array: {}", array[array.len() - 1]);
    println!("Last element of slice: {}", slice[slice.len() - 1]);
    println!("First element of array: {}", array.first().unwrap());
    println!("First element of slice: {}", slice.first().unwrap());
    println!("Last element of array: {}", array.last().unwrap());
    println!("Last element of slice: {}", slice.last().unwrap());
    println!("First element of array: {}", array.get(0).unwrap());
    println!("First element of slice: {}", slice.get(0).unwrap());
    println!("Last element of array: {}", array.get(array.len() - 1).unwrap());
    println!("Last element of slice: {}", slice.get(slice.len() - 1).unwrap());
    println!("First element of array: {}", array.first().unwrap());
    println!("First element of slice: {}", slice.first().unwrap());
    println!("Last element of array: {}", array.last().unwrap());
    println!("Last element of slice: {}", slice.last().unwrap());
    println!("First element of array: {}", array.first().unwrap());
    println!("First element of slice: {}", slice.first().unwrap());
    println!("Last element of array: {}", array.last().unwrap());
    println!("Last element of slice: {}", slice.last().unwrap());
    println!("First element of array: {}", array.first().unwrap());
    println!("First element of slice: {}", slice.first().unwrap());
    println!("Last element of array: {}", array.last().unwrap());
    println!("Last element of slice: {}", slice.last().unwrap());
    println!("First element of array: {}", array.first().unwrap());
    println!("First element of slice: {}", slice.first().unwrap());
    println!("Last element of array: {}", array.last().unwrap());
    println!("Last element of slice: {}", slice.last().unwrap());
    println!("First element of array: {}", array.first().unwrap());
}

fn test_string_slice() {
    let s = String::from("Hello, World!");
    let slice = &s[1..4];
    println!("String: {}", s);
    println!("Slice: {}", slice);
    println!("Length of string: {}", s.len());
    println!("Length of slice: {}", slice.len());
    println!("First character of string: {}", s.chars().next().unwrap());
    println!("First character of slice: {}", slice.chars().next().unwrap());
    println!("Last character of string: {}", s.chars().last().unwrap());
    println!("Last character of slice: {}", slice.chars().last().unwrap());
    println!("First character of string: {}", s.chars().nth(0).unwrap());
    println!("First character of slice: {}", slice.chars().nth(0).unwrap());
    println!("Last character of string: {}", s.chars().nth(s.len() - 1).unwrap());
    println!("Last character of slice: {}", slice.chars().nth(slice.len() - 1).unwrap());
    println!("First character of string: {}", s.chars().next().unwrap());
    println!("First character of slice: {}", slice.chars().next().unwrap());
    println!("Last character of string: {}", s.chars().last().unwrap());
    println!("Last character of slice: {}", slice.chars().last().unwrap());
    println!("First character of string: {}", s.chars().next().unwrap());
    println!("First character of slice: {}", slice.chars().next().unwrap());
    println!("Last character of string: {}", s.chars().last().unwrap());
    println!("Last character of slice: {}", slice.chars().last().unwrap());
    println!("First character of string: {}", s.chars().next().unwrap());
    println!("First character of slice: {}", slice.chars().next().unwrap());
    println!("Last character of string: {}", s.chars().last().unwrap());
    println!("Last character of slice: {}", slice.chars().last().unwrap());
    println!("First character of string: {}", s.chars().next().unwrap());

}





///
/// Test slice
/// slice are a reference to a contiguous sequence of elements in a collection
/// slice are a view into a collection
/// slice are a smart pointer
/// slice are a fat pointer
/// slice are a double word
/// slice are a pointer to the start of the slice and the length of the slice
/// 
pub fn test() {
    test_array_vs_slice();
    test_array_slice();
    test_string_slice();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        test();
    }
    
    #[test]
    fn test_test_array_vs_slice() {
        test_array_vs_slice();
    }

    #[test]
    fn test_test_array_slice() {
        test_array_slice();
    }

    #[test]
    fn test_test_string_slice() {
        test_string_slice();
    }
}