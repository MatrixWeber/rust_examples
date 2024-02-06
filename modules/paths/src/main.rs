mod my_module {
    pub fn my_function() {
        println!("This is my function from my module.");
    }
}

fn main() {
    crate::my_module::my_function(); // absolute path
    my_module::my_function();        // relative path
    self::my_func();                 // relative path
}

pub fn my_func() {
    println!("This is my internal function.");
}
