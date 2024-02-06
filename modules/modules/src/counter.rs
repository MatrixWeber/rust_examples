pub mod counter_module {
    fn private_counter_function() {
        println!("This is a private counter function.");
    }
    pub fn public_counter_function() {
        println!("This is a public counter function.");
        private_counter_function();
    }
    pub mod limit_counter {
        pub fn limit_counter_function() {
            println!("This is a public limit counter function.");
        }
    }
}