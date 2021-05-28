use std::fs;
use regex::Regex;

const special_method_call: &str  = "InitAndEnableFeature";

pub fn extract_class_strings_from_file(test_file: &str) -> Vec<String> {
    let f = fs::read_to_string(test_file);
    let full_file_str = f.unwrap();
    // Get the classes from the file using this regex pattern with the corresponding flags
    let re = Regex::new(r"((?sm)^class.*?^\};)").unwrap();
    let mut classes: Vec<String> = vec![];
    for cap in re.captures_iter(&full_file_str) {
        // Make a copy of the captured class string
        classes.push(String::from(&cap[0]));
    }
    return classes;
}

pub fn find_relevant_classes(class_str: &String) -> bool {
    return class_str.matches(special_method_call).count() == 1;
}

pub fn get_feature(class_str: &String) -> String  {
    let dog = format!(r"{}\(((?s).*?)\)", special_method_call);
    let re = Regex::new(&dog).unwrap();
    let result: String = String::from(&(re.captures_iter(&class_str).next().unwrap()[1]));
    return result;
}

// pub fn mutate_class(class_str: &String) -> bool {
    
// }