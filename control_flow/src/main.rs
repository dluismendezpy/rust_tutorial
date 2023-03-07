mod control_flow;

fn main() {
    control_flow::handling_conditions();
    control_flow::handle_if_let_statement();
    println!("The result is: {}", control_flow::handle_loops(10));
    control_flow::handle_multiple_loops();
    control_flow::handle_while();
    control_flow::handle_loop_for();
}
