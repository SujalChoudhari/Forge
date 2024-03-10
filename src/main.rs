use forger::Forger;
use logging::start;

pub mod argparser;
pub mod commander;
pub mod filehandler;
pub mod forger;
pub mod interpreter;
pub mod logging;
pub mod parser;
pub mod variables;

fn main() {
    start();
    let mut forger: Forger = Forger::new();
    forger.collect();
    forger.engrave();
    forger.forge();
    forger.quench();
}
