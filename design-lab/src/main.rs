// Singleton Logger Example
// This example demonstrates a singleton logger that can be used to log messages

type TimeSpend = i64;
type TimeLimit = i64;

mod struct_test;

use struct_test::test;
use std::option::Option;

enum TestResult {
    Passed(TimeSpend),
    Failed(TimeSpend),
    Stipped(TimeSpend),
    Stucked(TimeSpend),
    Invalid,
}

impl Clone for TestResult {
	fn clone(&self) -> TestResult {
			match self {
				TestResult::Passed(x) => TestResult::Passed(*x),
				TestResult::Failed(x) => TestResult::Failed(*x),
				TestResult::Stipped(x) => TestResult::Stipped(*x),
				TestResult::Stucked(x) => TestResult::Stucked(*x),
				TestResult::Invalid => TestResult::Invalid,
			}
	}
}

struct Test {
    test:fn () -> i32,
    name:String,
    shoud_skip:bool,
    result:TestResult,
    test_time_limit:TimeLimit,
}

impl Test {
    fn new(test:fn () -> i32, name:&String, should_skip:bool) -> Test {
        Test {
            test:test,
            name:name.clone(),
            shoud_skip:should_skip,
            result: TestResult::Invalid,
						test_time_limit:60 * 60,
        }
    }
    fn run(&mut self) {
        if !self.shoud_skip {
            println!("Running test:{}", self.name);
            /*
             * TODO: We should use a different thread to run the test
             *       and wait for the result for a given time.
             *       If the test does not return in the given time,
             *       we should kill the thread and return a failure.
             *       This is a simple implementation.
             */
            let start_time = std::time::Instant::now();
            let ret_status = (self.test)();
            let elapsed_time = start_time.elapsed();
            match ret_status {
                0 => {
                    self.result = TestResult::Passed(elapsed_time.as_millis() as i64);
                    println!("Test {} passed in {} ms", self.name, elapsed_time.as_millis());
                }
                _ => {
                    self.result = TestResult::Failed(elapsed_time.as_millis() as i64);
                    println!("Test {} stucked in {} ms", self.name, elapsed_time.as_millis());
                }
            }
            println!("Test {} completed in {} ms", self.name, elapsed_time.as_millis());
        }
    }
}

struct ModuleInfo{
    module_name:String,
    module_version:String,
    module_description:String,
    module_author:String,
    module_date:String,
    module_license:String,
}

impl ModuleInfo {
    fn new(module_name:&str, module_version:&str, module_description:&str, module_author:&str, module_date:&str, module_license:&str) -> ModuleInfo {
        ModuleInfo {
            module_name:String::from(module_name),
            module_version:String::from(module_version),
            module_description:String::from(module_description),
            module_author:String::from(module_author),
            module_date:String::from(module_date),
            module_license:String::from(module_license),
        }
    }
    fn getModuleInfo(&self) -> String {
        format!("Module Name: {}\nModule Version: {}\nModule Description: {}\nModule Author: {}\nModule Date: {}\nModule License: {}",
            self.module_name,
            self.module_version,
            self.module_description,
            self.module_author,
            self.module_date,
            self.module_license)
    }
}


struct Tests {
    module:Option<ModuleInfo>,
    tests: Vec<Test>,
}

impl Tests {
    fn new() -> Tests{
        Tests {
            module: None,
            tests: Vec::new(),
        }
    }
    fn add_test(&mut self, test:Test) {
        self.tests.push(test);
    }
    fn run_tests(&mut self) {
        match &self.module {
					Some(_module) =>  println!("Running tests of module {}", _module.getModuleInfo()),
					_ => println!("Running tests of module {}", "Unkown"),
				}
        for test in &mut self.tests {
            test.run();
        }
    }
    fn get_test_results(&self) -> Vec<TestResult> {
        let mut results = Vec::new();
        for test in &self.tests {
            results.push(test.result.clone());
        }
        results
    }
}

enum TestTime {
    Small = 1000,
    Medium = 5000,
    Large = 10000,
    ExtraLarge = 20000,
}

fn main() {
    let mut tests:Tests = Tests::new();
    tests.add_test(	Test{
														test:struct_test::test as _ ,
														name:String::from("test_struct"),
														shoud_skip:false,
														result:TestResult::Invalid,
														test_time_limit:15,
		});
    println!("Running tests...");
    tests.run_tests();
    println!("All tests completed.");

}
