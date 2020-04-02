mod ownership;
mod references_borrowing;
mod slice;

fn main() {
    // OwnerShips
    println!("\n********** Ownership **********\n");
    ownership::mutable();
    ownership::binding();
    ownership::own_functions();
    ownership::return_values_scope();

    // references and borrowing
    println!("\n********** References and borrowing **********\n");
    references_borrowing::fix_calculate_len();
    references_borrowing::references_mutables();
    references_borrowing::dangle_references();

    // Slice
    println!("\n********** Slice **********\n");
    slice::main();
}