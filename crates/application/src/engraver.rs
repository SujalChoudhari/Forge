use argparser::Arguments;
use constants::{LINUX_STRING, MAC_STRING, OS_VARIABLE_NAME, WIN_STRING};
use interpreter::get_variables;
use variable::Variables;
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Engraver {
    os: String,
}

impl Engraver {
    pub fn new() -> Self {
        Engraver {
            os: if cfg!(target_os = "windows") {
                WIN_STRING.to_string()
            } else if cfg!(target_os = "macos") {
                MAC_STRING.to_string()
            } else {
                LINUX_STRING.to_owned()
            },
        }
    }

    pub fn engrave(&mut self, arguments: &Arguments, variables: &mut Variables, job: &Yaml) {
        self.engrave_args_in_variables(arguments, variables);
        self.engrave_declared_vars_in_variables(variables, job);
        self.engrave_default_vars_in_variables(variables);
    }

    fn engrave_args_in_variables(&self, arguments: &Arguments, variables: &mut Variables) {
        variables.add_from_hash(&arguments.keyword_arguments);
    }

    fn engrave_declared_vars_in_variables(&self, variables: &mut Variables, job: &Yaml) {
        variables.add_from_hash(&get_variables(job));
    }

    fn engrave_default_vars_in_variables(&self, variables: &mut Variables) {
        // add the default args
        variables.add(OS_VARIABLE_NAME.to_string(), &self.os);
    }

    
}
