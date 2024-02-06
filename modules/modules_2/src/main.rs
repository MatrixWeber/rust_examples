mod counter;
mod limit_counter; // rust is seeking for a counter.rs file

use limit_counter::limit_counter::limit_counter_function as lcf;
use crate::counter::counter_module::*;

fn main() {
    lcf();
    public_counter_function();
    // private_counter_function(); Compiler failure
}
