
#[cfg(test)]
mod test_parse_class {
    use refactortests::utils::*;

    #[test]
    fn test_get_class_only_with_regex() {
        let c = extract_class_strings_from_file("tests/fixtures/test_class.cc");
        let d: Vec<&String> = c.iter().filter(|&x| find_relevant_classes(x)).collect();
        for i in c {
            if find_relevant_classes(&i) {
                println!("{:?}", get_feature(&i));
            }
        }
    }

    #[test]
    fn test_string_matches() {
        let dog = String::from("hello world this is a world hello");
        println!("{:?}", dog.matches("hello").count());
    }
}
   
