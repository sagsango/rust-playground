use std::string::String;
use std::vec::Vec;

fn copy_and_return(vector: &mut Vec<String>, value: &str) -> String {
    vector.push(String::from(value));
    value.to_string()
}

///
/// Copy and return the value
/// This is a miscelleneous test
/// 
pub fn test() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}
