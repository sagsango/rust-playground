mod hashMapTest;
mod hashSetTest;
mod VecTest;
mod VecDequeTest;
mod StringTest;
mod BTreeMapTest;
mod BTreeSetTest;
mod LinkedListTest;
mod BinaryHeapTest;
mod BitSetTest;
mod enumTest;
mod structTest;
mod functionArgsTest;
mod sliceTest;
mod graphTest;
mod stdinTest;
mod miscelleneousTest;
mod matchTest;
mod traitTest;

/* Memory managrment */
mod staticTest;
mod borrowTest;
mod concurrencyTest;
mod copyTraitTest;
mod unsafeTest;
mod glibcTest;

fn main() {
    // use hashmap to put the test name and test function
    let mut tests = std::collections::HashMap::new();
    tests.insert("hashMapTest", hashMapTest::test as fn());
    tests.insert("hashSetTest", hashSetTest::test as fn());
    tests.insert("VecTest", VecTest::test as fn());
    tests.insert("VecDequeTest", VecDequeTest::test as fn());
    tests.insert("StringTest", StringTest::test as fn());
    tests.insert("BTreeMapTest", BTreeMapTest::test as fn());
    tests.insert("BTreeSetTest", BTreeSetTest::test as fn());
    tests.insert("LinkedListTest", LinkedListTest::test as fn());
    tests.insert("BinaryHeapTest", BinaryHeapTest::test as fn());
    tests.insert("BitSetTest", BitSetTest::test as fn());
    tests.insert("enumTest", enumTest::test as fn());
    tests.insert("structTest", structTest::test as fn());
    tests.insert("functionArgsTest", functionArgsTest::test as fn());
    tests.insert("sliceTest", sliceTest::test as fn());
    tests.insert("graphTest", graphTest::test as fn());
    tests.insert("stdinTest", stdinTest::test as fn());
    tests.insert("miscelleneousTest", miscelleneousTest::test as fn());
    tests.insert("staticTest", staticTest::test as fn());
    tests.insert("borrowTest", borrowTest::test as fn());
    tests.insert("matchTest", matchTest::test as fn());
    tests.insert("traitTest", traitTest::test as fn());
    tests.insert("concurrencyTest", concurrencyTest::test as fn());
    tests.insert("copyTraitTest", copyTraitTest::test as fn());
    tests.insert("unsafeTest", unsafeTest::test as fn());
    tests.insert("glibcTest", glibcTest::test as fn());

    // run all the tests
    for (name, test) in tests.iter() {
        println!("Running test: {}", name);
        test();
        println!("Test: {} passed", name);
    }

    // run only one test
    let test = tests.get("matchTest").unwrap();
    test();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}