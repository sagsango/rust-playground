// Here we we see how to use c library - glibc
extern crate libc;

///
/// Test the glibc functionality
/// We are testing the following:
/// - stdout
/// - stdin
/// - fopen
/// - fclose
/// - malloc
/// - free
/// - getcwd
/// - getpid
/// - getppid
/// - getuid
/// - getgid
/// - syscall
/// - write
/// 
pub fn test() {
    // stdout
    unsafe {
        libc::printf("Hello, World!\n".as_ptr() as *const i8);
    }
    // stdin
    let mut input = String::new();
    unsafe {
        libc::printf("Enter a number: ".as_ptr() as *const i8);
        std::io::stdin().read_line(&mut input).unwrap();
    }
    
    println!("You entered: {}", input);
    // fopen
    let file = unsafe {
        libc::fopen("test.txt".as_ptr() as *const i8, "w".as_ptr() as *const i8)
    };
    if file.is_null() {
        println!("Failed to open file");
    } else {
        println!("File opened successfully");
    }
    // fclose
    let result = unsafe {
        libc::fclose(file)
    };
    if result == 0 {
        println!("File closed successfully");
    } else {
        println!("Failed to close file");
    }

    // malloc
    let ptr = unsafe {
        libc::malloc(10)
    };
    if ptr.is_null() {
        println!("Failed to allocate memory");
    } else {
        println!("Memory allocated successfully");
    }
    // free
    unsafe {
        libc::free(ptr);
    }
    println!("Memory freed successfully");
    // getcwd
    let mut buffer = [0u8; 100];
    let cwd = unsafe {
        libc::getcwd(buffer.as_mut_ptr() as *mut i8, buffer.len())
    };
    if cwd.is_null() {
        println!("Failed to get current working directory");
    } else {
        unsafe {
            println!("Current working directory: {:?}", std::ffi::CStr::from_ptr(cwd));
        }
    }
    // systems call - getpid
    let pid = unsafe {
        libc::getpid()
    };
    println!("Process ID: {}", pid);
    // systems call - getppid
    let ppid = unsafe {
        libc::getppid()
    };
    println!("Parent Process ID: {}", ppid);
    // systems call - getuid
    let uid = unsafe {
        libc::getuid()
    };
    println!("User ID: {}", uid);
    // systems call - getgid
    let gid = unsafe {
        libc::getgid()
    };
    println!("Group ID: {}", gid);

    // raw systems call 
    let result = unsafe {
        libc::syscall(1, 0, 0, 0, 0, 0, 0)
    };
    println!("Result of syscall: {}", result);

    // write to file discriptor 1 (stdout)
    let result = unsafe {
        libc::write(1, "Hello, World from syscall!\n".as_ptr() as *const libc::c_void, 13)
    };
    println!("Result of write: {}", result);
}

// Run the test
#[test]
fn test_test() {
    test();
}
