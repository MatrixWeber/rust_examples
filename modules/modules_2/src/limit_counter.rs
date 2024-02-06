pub mod limit_counter {
    pub fn limit_counter_function() {
        println!("This is a public limit counter function.");
        crate::counter::counter_module::public_counter_function();
    }
}