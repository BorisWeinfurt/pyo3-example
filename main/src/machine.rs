
// This can be avoided later, it is meant as a simplistic demo
pub const METHODS: &[&str] = &[
    "fn required_method1 (& self , arg1 : i32 , arg2 : String) -> bool",
    "fn required_method2 (& self , arg1 : String , arg2 : bool)",
];

// Example machine trait
pub trait MachineTrait {
    fn required_method1(&self, arg1: i32, arg2: String) -> bool;

    fn required_method2(&self, arg1: String, arg2: bool);
}

// Representation of a standard elvis rust based machine
pub struct StandardMachine;
impl StandardMachine {
    pub fn new() -> Self {
        Self
    }
}
impl MachineTrait for StandardMachine {
    fn required_method1(&self, arg1: i32, arg2: String) -> bool {
        println!("Hi from required method 1");
        return true;
    }

    fn required_method2(&self, arg1: String, arg2: bool) {
        println!("Hi from required method 2");
    }
}
