pub fn load_args() -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    for individual_args in std::env::args() {
        args.push(individual_args);
    }
    args
}
