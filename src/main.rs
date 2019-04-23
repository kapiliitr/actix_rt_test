pub mod service;
pub mod tests;

fn main() {
    service::run_sysrunner();
    service::run_arbiter();
    service::run_tokio();
}