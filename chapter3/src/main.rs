mod datatypes;
mod loops;
mod mutability;

fn main() {
    println!("Hello, world!");
    mutability::print_numbers();
    datatypes::do_math(6, 7);
    datatypes::bools();
    datatypes::strings();
    datatypes::tuples();
    datatypes::arrays();
    loops::ifs();
    loops::loops();
}
