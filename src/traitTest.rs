
/*
1. Definition and Purpose:
A trait in Rust is a collection of methods that types can implement. Traits are used to define shared behavior across different types. They are similar to interfaces in other languages, like Java or C#.

2. Method Signatures and Default Implementations:
Traits can include method signatures (methods without bodies) that the implementing type must define. Traits can also provide default method implementations, which implementing types can override. Take note that traits can not have fields.

3. Trait Implementation:
A type implements a trait by providing concrete implementations for the trait’s methods. This is done using the impl TraitName for TypeName syntax.

4. Trait Bounds:
Traits can be used as bounds in generic functions or structs, ensuring that the generic type implements a particular trait. For example, fn print_name<T: Person>(t: T) ensures that T implements the Person trait.

5. Dynamic Dispatch:
Traits can be used with dynamic dispatch via dyn Trait. This allows for runtime polymorphism, where the exact method implementation to call is determined at runtime. However, this comes with a performance cost compared to static dispatch.

6. Object Safety:
Not all traits can be used as trait objects (dyn Trait). For a trait to be "object-safe," it must meet certain conditions, such as not having methods that return Self or methods that have generic type parameters.

7. Trait Inheritance:
Traits can inherit from other traits, allowing for a hierarchy of traits. This means that a trait can require the implementation of another trait.

8. Associated Types:
Traits can define associated types, which are types that are associated with the trait and can vary depending on the implementation. For example, the Iterator trait has an associated type Item that represents the type of value yielded by the iterator.

9. Using Traits for Abstraction:
Traits are commonly used to create abstractions in Rust. For example, the Iterator trait abstracts over different types of iterators, and the Display trait abstracts over different types that can be converted to a string.

10. Deriving Traits:
Rust provides the ability to automatically implement certain traits for types using the #[derive] attribute. Commonly derived traits include Clone, Copy, Debug, PartialEq, and Eq.

11. Orphan Rule:
Rust enforces the "orphan rule," which means you can only implement a trait for a type if either the trait or the type is defined in the current crate. This prevents potential conflicts between implementations in different crates.

12. Auto Traits:
Some traits, like Send and Sync, are automatically implemented for types when certain conditions are met. These are known as "auto traits."
 */
trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Dog {
    name: &'static str,
}

impl Dog {
    fn new(name: &'static str) -> Self {
        Dog { name }
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Woof!", self.name());
    }
}

struct Cat {
    name: &'static str,
}

impl Cat {
    fn new(name: &'static str) -> Self {
        Cat { name }
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Meow!", self.name());
    }
}

fn test_animal(animal: &dyn Animal) {
    animal.talk();
}
/*
// This does not work
// error[E0038]: the trait Animal cannot be made into an object
// --> src/traitTest.rs:56:17
// 
// 56 |     test_animal(&cat);
//    |                 ^^^^ Animal cannot be made into an object

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Dog {
    name: &'static str,
}

impl Animal for Dog {
    fn create(name: &'static str) -> Self {
        Dog { name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Woof!", self.name());
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn create(name: &'static str) -> Self {
        Cat { name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Meow!", self.name());
    }
}

fn test_animal(animal: &dyn Animal) {
    animal.talk();
}
*/

fn test1() {
    let dog = Dog::new("Dog");
    dog.talk();
    let cat = Cat::new("Cat");
    cat.talk();
    test_animal(&dog);
    test_animal(&cat);
}

// another trait
trait Person {
    // Trait can not have fields, only methods; point 2
    // let name: &'static str;;
    // let age: u32;
    // let height: f32;
    // let weight: f32;
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn age(&self) -> u32;
    fn height(&self) -> f32;
    fn weight(&self) -> f32;
}

struct Student {
    name: &'static str,
    age: u32,
    height: f32,
    weight: f32,
}

impl Student {
    fn new(name: &'static str) -> Self {
        Student {
            name,
            age: 0,
            height: 0.0,
            weight: 0.0,
        }
    }
}

impl Person for Student {
    fn new(name: &'static str) -> Self {
        Student {
            name,
            age: 0,
            height: 0.0,
            weight: 0.0,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn age(&self) -> u32 {
        self.age
    }
    fn height(&self) -> f32 {
        self.height
    }
    fn weight(&self) -> f32 {
        self.weight
    }
}

struct Teacher {
    name: &'static str,
    age: u32,
    height: f32,
    weight: f32,
}

impl Teacher {
    fn new(name: &'static str) -> Self {
        Teacher {
            name,
            age: 0,
            height: 0.0,
            weight: 0.0,
        }
    }
}

impl Person for Teacher {
    fn new(name: &'static str) -> Self {
        Teacher {
            name,
            age: 0,
            height: 0.0,
            weight: 0.0,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn age(&self) -> u32 {
        self.age
    }
    fn height(&self) -> f32 {
        self.height
    }
    fn weight(&self) -> f32 {
        self.weight
    }
}

fn test2() {
    let student = Student::new("Student");
    let teacher = Teacher::new("Teacher");
    println!("{} is {} years old", student.name(), student.age());
    println!("{} is {} years old", teacher.name(), teacher.age());
}


// Derived traits from another traits Person
trait Employee: Person {
    fn salary(&self) -> f32;
}

struct Engineer {
    name: &'static str,
    age: u32,
    height: f32,
    weight: f32,
    salary: f32,
}

impl Engineer {
    fn new(name: &'static str) -> Self {
        Engineer {
            name,
            age: 0,
            height: 0.0,
            weight: 0.0,
            salary: 0.0,
        }
    }
}

impl Person for Engineer {
    fn new(name: &'static str) -> Self {
        Engineer {
            name,
            age: 0,
            height: 0.0,
            weight: 0.0,
            salary: 0.0,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn age(&self) -> u32 {
        self.age
    }
    fn height(&self) -> f32 {
        self.height
    }
    fn weight(&self) -> f32 {
        self.weight
    }
}

impl Employee for Engineer {
    fn salary(&self) -> f32 {
        self.salary
    }
}

fn test3() {
    let engineer = Engineer::new("Engineer");
    println!("{} is {} years old", engineer.name(), engineer.age());
    println!("{}'s salary is {}", engineer.name(), engineer.salary());
}

/*
// Can not Deriv trait from struct Engineer
trait Manager: Engineer {
    fn team_size(&self) -> u32;
}
*/
// create a struct and then drive the trait
struct Manager {
    name: &'static str,
    age: u32,
    height: f32,
    weight: f32,
    salary: f32,
    team_size: u32,
}

impl Manager {
    fn new(name: &'static str) -> Self {
        Manager {
            name,
            age: 0,
            height: 0.0,
            weight: 0.0,
            salary: 0.0,
            team_size: 0,
        }
    }
}

impl Person for Manager {
    fn new(name: &'static str) -> Self {
        Manager {
            name,
            age: 0,
            height: 0.0,
            weight: 0.0,
            salary: 0.0,
            team_size: 0,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn age(&self) -> u32 {
        self.age
    }
    fn height(&self) -> f32 {
        self.height
    }
    fn weight(&self) -> f32 {
        self.weight
    }
}

impl Employee for Manager {
    fn salary(&self) -> f32 {
        self.salary
    }
}

impl Manager {
    fn team_size(&self) -> u32 {
        self.team_size
    }
}

fn test4() {
    let manager = Manager::new("Manager");
    println!("{} is {} years old", manager.name(), manager.age());
    println!("{}'s salary is {}", manager.name(), manager.salary());
    println!("{}'s team size is {}", manager.name(), manager.team_size());
}

// trait with default implementation
trait Greet {
    fn greet(&self) {
        println!("Hello!");
    }
}

struct Person1 {
    name: &'static str,
}

impl Greet for Person1 {}

impl Person1 {
    fn new(name: &'static str) -> Self {
        Person1 { name }
    }
}

fn test5() {
    let person = Person1::new("Person");
    person.greet();
}


// dynamic dispatch
trait Greet1 {
    fn greet(&self);
}

struct Person2 {
    name: &'static str,
}

impl Greet1 for Person2 {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

struct Dog1 {
    name: &'static str,
}

impl Greet1 for Dog1 {
    fn greet(&self) {
        println!("Woof! I'm {}", self.name);
    }
}

fn test6() {
    let person = Person2 { name: "Alice" };
    let dog = Dog1 { name: "Rover" };

    let greeters: Vec<&dyn Greet1> = vec![&person, &dog];

    for greeter in greeters {
        greeter.greet();
    }
}

// trait which deals with associated types
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

struct Conunter_u64 {
    count: u64,
}

impl Iterator for Conunter_u64 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}



fn test7() {
    let mut counter = Counter { count: 0 };

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let mut counter_u64 = Conunter_u64 { count: 0 };
    assert!(counter_u64.next().is_some());
    assert!(counter_u64.next().is_some());
    assert!(counter_u64.next().is_some());
    assert!(counter_u64.next().is_some());
    assert!(counter_u64.next().is_some());
    assert!(counter_u64.next().is_none());


}

// trait with associated types
trait Complex {
    type Real;
    type Imaginary;
    fn new(real: Self::Real, imaginary: Self::Imaginary) -> Self;
    fn real(&self) -> Self::Real;
    fn imaginary(&self) -> Self::Imaginary;
}

struct ComplexNumber {
    real: f64,
    imaginary: f64,
}

impl Complex for ComplexNumber {
    type Real = f64;
    type Imaginary = f64;

    fn new(real: Self::Real, imaginary: Self::Imaginary) -> Self {
        ComplexNumber { real, imaginary }
    }

    fn real(&self) -> Self::Real {
        self.real
    }

    fn imaginary(&self) -> Self::Imaginary {
        self.imaginary
    }
}

fn test8() {
    let complex = ComplexNumber::new(3.0, 4.0);
    println!("Real: {}", complex.real());
    println!("Imaginary: {}", complex.imaginary());
}

pub fn test() {
    test1(); // test Animal trait
    test2(); // test Person trait
    test3(); // test Employee trait
    test4(); // test Manager trait
    test5(); // test Greet trait
    test6(); // test Greet1 trait
    test7(); // test Iterator trait
    test8(); // test Complex trait
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait() {
        test();
    }

}
