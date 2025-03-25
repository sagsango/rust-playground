
mod debugArc;
mod debugArcMultiStruct;
mod debugNestedObjectRefcounts;

fn main() {
    debugArc::test();
		debugArcMultiStruct::test();
		debugNestedObjectRefcounts::test();
    println!("Hello, world!");
}
