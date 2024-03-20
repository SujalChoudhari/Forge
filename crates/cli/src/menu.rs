use constants::APP_FILENAME_DEFAULT_PATH;
use logger::Logger;
use yaml_rust::Yaml;

fn get_all_recipe_names(book: Yaml) -> Vec<String> {
    match book {
        Yaml::Hash(hash) => hash
            .keys()
            .into_iter()
            .map(|f| match f {
                Yaml::String(str) => str.to_owned(),
                _ => "help".to_string(),
            })
            .collect::<Vec<String>>(),
        _ => {
            vec![]
        }
    }
}

pub fn run_menu_handler() -> String {
    let forge = parser::load_forge(APP_FILENAME_DEFAULT_PATH);
    let intermidiate_names = get_all_recipe_names(forge);
    let all_names = intermidiate_names.iter().map(|f| f.as_str()).collect();

    let command = Logger::input_choice("Select the recipe to run:", all_names);
    command
}
