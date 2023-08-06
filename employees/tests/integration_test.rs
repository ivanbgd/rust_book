use std::collections::HashMap;

use employees::constants::*;
use employees::*;


#[test]
fn integration_test() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    println!("{}", HELP);
    help();

    let mut words: Vec<&str> = "add John Doe to Finance Dept".split_whitespace().collect();
    add_employee(words, &mut employees);
    words = "add Mark Best to Finance Dept".split_whitespace().collect();
    add_employee(words, &mut employees);
    words = "add Pearl to Engineering".split_whitespace().collect();
    add_employee(words, &mut employees);
    words = "add Jane Doe to Finance Dept".split_whitespace().collect();
    add_employee(words, &mut employees);

    words = "deptsort Finance Dept".split_whitespace().collect();
    print_and_sort_dept(words, &mut employees);

    let words = "remove John Doe from Finance Dept".split_whitespace().collect();
    remove_employee(&words, &mut employees);
    assert_eq!(employees["Finance Dept"],
        vec!["Jane Doe".to_string(), "Mark Best".to_string()]
    );

    assert!(employees.contains_key("Finance Dept"));
    assert!(employees.contains_key("Engineering"));

    let words = "remove Finance Dept".split_whitespace().collect();
    remove_dept(&words, &mut employees);
    assert!(!employees.contains_key("Finance Dept"));
    assert!(employees.contains_key("Engineering"));
}
