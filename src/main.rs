pub mod service;
pub mod tests;

fn main() {
    service::run_tokio();
    service::run_sysrunner();
    service::run_arbiter();
}