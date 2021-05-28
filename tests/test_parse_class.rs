
#[cfg(test)]
mod test_parse_class {
    use std::fs;
    use std::env;
    use regex::Regex;
    // #[test]
    // fn test_something() {
    //     let my_class = test_class_fixture();
    //     println!("{}", my_class);
    // }

    #[test]
    fn test_get_class_only_with_regex() {
        let f = fs::read_to_string("tests/fixtures/test_class.cc");
        // let f = fs::read_to_string("tests/fixtures/test_error.cc");
        let full_file_str = f.unwrap();
        // let re = Regex::new(r"^class.*scoped_feature_list_.InitAndEnableFeature.*\};").unwrap();
        let re = Regex::new(r"((?sm)^class.*?^\};)").unwrap();
        // assert!(re.is_match(&full_file_str));
        // assert!(re.is_match(&full_file_str));
        // assert!(re.is_match(&full_file_str));
        for cap in re.captures_iter(&full_file_str) {
            println!("-----------------------------------------------------------------------\n");
            println!("{}", &cap[0]);
        }
    }
}
   
   