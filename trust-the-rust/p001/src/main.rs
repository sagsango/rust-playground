struct Info{
    name:String,
    id:i32,
}

impl Info {
    fn new() -> Info {
        Info {
            name:String::from(""),
            id:-1,
        }
    }
    
    fn setName(&mut self, name:String) -> () {
        self.name = name;
    }
    
    fn setId(&mut self, id:i32) -> () {
        self.id = id;
    }
    
    fn printInfo(&self) {
        println!("name:{}, id:{}", self.name, self.id);
    }
}
fn main() {
    let mut info = Info::new();
    info.printInfo();
    info.setName(String::from("LaLa Land"));
    info.setId(25);
    info.printInfo();
    
    println!("Hello, world!");
}
