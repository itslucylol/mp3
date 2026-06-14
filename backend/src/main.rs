const LIBRARY_ROOT: &str = "./library";
pub mod routes;

mod axum;
fn main() {
    axum::main();
}
