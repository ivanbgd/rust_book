mod constants;

use std::collections::{BTreeSet, HashMap};
use std::io::stdin;

use crate::constants::*;


/// From [The Rust Book](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary):
///
/// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department
/// in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales”. Then let the user retrieve a list
/// of all people in a department or all people in the company by department, sorted alphabetically.
///
/// This is an improved version of the original requirements.
///
/// We are creating a department when adding the first employee to it.
/// Alternatively, we could have a command to just create a department,
/// and then adding employees would always add them to an existing department.
///
/// **Note:** `BTreeMap` is always sorted by keys, unlike `HashMap`, but we use `HashMap` here, as required.
/// `HashMap` cannot be sorted, at least not by using standard library,
/// but it can be sorted by using the crate [`itertools`](https://docs.rs/itertools/latest/itertools/).
fn employees() {
    // Keys in the hash map are department names, as those are unique.
    // Values are vectors of employee names, as those are not unique.
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    // Allocate memory for `line` only once
    let mut line = String::new();

    loop {
        println!("Enter command: ");
        stdin().read_line(&mut line).unwrap_or_default();
        // print!("[DEBUG] Command: {line}");  // for debugging
        if line == "\n" {
            break;
        }
        let words: Vec<_> = line.split_whitespace().collect();  // Vec<&str>
        // println!("[DEBUG] Split line: {words:?}; length = {}", words.len());  // for debugging
        let cmd = words[0].to_lowercase();
        match cmd.as_str() {
            HELP => help(),
            ADD => add_employee(words, &mut employees),
            DEPT => print_dept(words, &employees),
            DEPT_SORT => print_and_sort_dept(words, &mut employees),
            DEPTS => print_depts(&employees),
            ALL => print_all(&employees),
            ALL_BY_DEPT => print_all_by_dept(&employees),
            ALL_UNIQUE_PER_DEPT => print_all_unique_per_dept(&employees),
            REMOVE => remove_employee(&words, &mut employees),
            REMOVE_DEPT => remove_dept(&words, &mut employees),
            CLEAR => clear(&mut employees),
            STOP => break,
            _ => println!("Unrecognized command; try `help`"),
        };
        line.clear();  // Free up memory for `line`
    }

    println!("\n[DEBUG] Employees: {employees:#?}");  // for debugging
}

fn help() {
    println!("{HELP} {ADD} {DEPT} {DEPT_SORT} {DEPTS} {ALL} {ALL_BY_DEPT} \
    {ALL_UNIQUE_PER_DEPT} {REMOVE} {REMOVE_DEPT} {CLEAR} {STOP}");
}

/// **Add an employee to a department**
///
/// Employee and department name can consist of multiple words.
/// We can wrap employee name and/or department name in single or double quotes,
/// but we don't have to use any quotes at all.
/// The department doesn't have to exist in advance. If it doesn't exist, it will be created on this occasion.
fn add_employee(words: Vec<&str>, employees: &mut HashMap<String, Vec<String>>) {
    if (words.len() < 4) || !words.contains(&"to") {
        println!("The Add command: {ADD} 'first_name mid_name last_name' to 'dept name'");
        return;
    }

    let to_pos = words.iter().position(|&r| r == "to").expect("The ADD command must contain \"to\".");

    let name = words[1..to_pos].join(" ").trim_matches(|c| c == '\'' || c == '\"').trim().to_string();

    {
        // These two variants are equivalent, and only work in case dept name consists of exactly one word.
        // This is kept only as an example, but not otherwise used.
        let _dept = words.last().unwrap_or(&"").to_owned();
       let _dept = words[words.len() - 1];
    }
    let dept = words[to_pos+1..].join(" ").trim_matches(|c| c == '\'' || c == '\"').trim().to_string();
    let dept = employees.entry(dept).or_insert(Vec::new());

    dept.push(name);
}

/// **Print all people in the given department in alphabetical order**
///
/// Only the display is sorted, and not the collection itself.
fn print_dept(words: Vec<&str>, employees: &HashMap<String, Vec<String>>) {
    // We only sort for printing in this case, and not in the original "employees" hash map.
    // `employees` is an immutable reference, so this should be considered an original and safer
    // implementation than the alternative one, which is right below.
    // In the caller, for the parameter `employees`, we can use a mutable reference in either implementation.
    if words.len() < 2 {
        println!("The Dept command: {DEPT} 'dept name'");
        return;
    }

    let dept = words[1..].join(" ").trim_matches(|c| c == '\'' || c == '\"').trim().to_string();

    match employees.get(&dept) {
        Some(_) => println!("The department \"{dept}\" contains:"),
        None => {
            println!("The department \"{dept}\" doesn't exist.");
            return;
        },
    }

    // Here, `unwrap()` would work instead of `unwrap_or(&aux)`, because we handled the case of non-existing
    // department (key in the `employees` hash map) above, returning from this function early in that case.
    // Still, as using `unwrap()` is not recommended, we wanted to comply with that recommendation, because,
    // in general case, code can be refactored, and the above check and early return may go away.
    // We want to make code as safe as possible, and as resilient to changes as possible.
    // As it stands, the `aux` allocation will never happen, because `get()` will always return something
    // at this point, so there is no run-time penalty whatsoever, but even if somebody removes the early return
    // in the future, the current code will continue to function properly, without panicking.
    // Instead of `to_owned()` at the end, we can use `clone()` - it's the same in this case, because
    // we don't want to sort the collection really, but only for printing.
    let aux = Vec::<String>::new();
    let mut dept_employees = employees.get(&dept).unwrap_or(&aux).to_owned();
    dept_employees.sort_unstable();
    for employee in dept_employees {
        println!("{employee}");
    }
}

/// **Print all people in the given department in alphabetical order**
///
/// Only the display is sorted, and not the collection itself.
/// This is an alternative implementation, probably less safe than the original one.
fn print_dept_alternative(words: Vec<&str>, employees: &mut HashMap<String, Vec<String>>) {
    #![allow(unused)]
    // We only sort for printing in this case, and not in the original "employees" hash map.
    // `employees` is a mutable reference, so this should be considered a less safe
    // implementation than the original one, which is right above.
    // In the caller, for the parameter `employees`, we can use a mutable reference in either implementation.
    // This implementation is similar to `print_and_sort_dept()`, i.e., it uses pattern matching,
    // but that requires the parameter `employees` to be a mutable reference,
    // because `dept_employees` needs to be mutable, as we want to sort it.
    // We are then using a clone, so we don't modify the original data structure,
    // but only modify (sort) the clone for printing purposes.
    if words.len() < 2 {
        println!("The Dept command: {DEPT} 'dept name'");
        return;
    }

    let dept = words[1..].join(" ").trim_matches(|c| c == '\'' || c == '\"').trim().to_string();

    // `cloned()` copies the department (the value in the `employees` hash map for the given key), so we can
    // operate on the copy and sort it. The original hash map `employees` remains intact.
    if let Some(mut dept_employees) = employees.get_mut(&dept).cloned() {
        dept_employees.sort_unstable();
        println!("The department \"{dept}\" contains:");
        for employee in dept_employees {
            println!("{employee}");
        }
    } else {
        println!("The department \"{dept}\" doesn't exist.");
    }
}

/// **Print and sort all people in the given department in alphabetical order**
///
/// Sorts employees in the department first, so they stay sorted, until a new entry.
/// We can wrap the department name in single or double quotes, but we don't have to use any quotes at all.
fn print_and_sort_dept(words: Vec<&str>, employees: &mut HashMap<String, Vec<String>>) {
    // This function is sorting employees in the department, in the original hash map "employees" (every time).
    // The alternative is to clone the dept employees and sort them only locally here, for the printing purposes,
    // but that's not an improvement in performance; it's even downgrade as we have to clone first, every time.
    // We do sort the original hash map, which doesn't hurt semantically, but we at least don't waste time
    // on unnecessary cloning.
    // This is why we need mutable reference to "employees". In case of cloning, we wouldn't need the reference
    // to be mutable (or in case we didn't want to print in sorted order).
    // Cloning or converting to owned in this case would actually NOT sort the original collection,
    // but would only sort the display, just like in `print_dept()`!
    if words.len() < 2 {
        println!("The Dept Sort command: {DEPT_SORT} 'dept name'");
        return;
    }

    let dept = words[1..].join(" ").trim_matches(|c| c == '\'' || c == '\"').trim().to_string();

    if let Some(dept_employees) = employees.get_mut(&dept) {
        dept_employees.sort_unstable();
        println!("The department \"{dept}\" contains:");
        for employee in dept_employees {
            println!("{employee}");
        }
    } else {
        println!("The department \"{dept}\" doesn't exist.");
    }
}

/// **Print all departments in alphabetical order**
///
/// Doesn't sort the hash map by keys, as it cannot be sorted anyway.
/// If we need it sorted, then we should use `BTreeMap` instead of `HashMap`.
/// But, we don't need it sorted.
/// So, this just sorts the printout.
fn print_depts(employees: &HashMap<String, Vec<String>>) {
    let mut depts = employees.iter().collect::<Vec<_>>();
    depts.sort_unstable();
    println!("Departments:");
    for (dept, _) in &depts {
        println!("{dept}");
    }
}

/// **Print all departments in alphabetical order**
///
/// Alternative implementation.
/// Doesn't sort the hash map by keys, as it cannot be sorted anyway.
/// If we need it sorted, then we should use `BTreeMap` instead of `HashMap`.
/// But, we don't need it sorted.
/// So, this just sorts the printout.
fn print_depts_alt(employees: &HashMap<String, Vec<String>>) {
    #![allow(unused)]
    let mut depts = employees.iter().collect::<Vec<_>>();
    depts.sort_unstable();
    println!("Departments: {:#?}", depts);
}

/// **Print all employees in company and their department in alphabetical order by employee name**
///
/// Only sorts the printout, as `HashMap` cannot be sorted, at least not by using standard library.
/// `HashMap` can be sorted by using the crate [`itertools`](https://docs.rs/itertools/latest/itertools/).
///
/// Explore the following crates as alternatives to this implementation, instead of `Vector` that's used here:
/// - [`BTreeMultiMap`](https://docs.rs/btreemultimap/latest/btreemultimap/), used as `MultiSet`,
/// - [`ord-by-set`](https://lib.rs/crates/ord-by-set) Not sure it supports full equivalence of keys; perhaps if used
/// as `BTreeMap` (see the second bullet point).
///
/// These data structures are always sorted, i.e., they re-sort themselves at each insertion or removal.
fn print_all(employees: &HashMap<String, Vec<String>>) {
    let mut all_employees = Vec::new();  // all_employees: Vec<(&String, &String)> <=> [(employee, dept)]
    for (dept, employees) in employees.iter() {
        for employee in employees {
            all_employees.push((employee, dept));
        }
    }
    all_employees.sort_unstable();

    println!("Employee, Department:");
    let underline = "=".repeat("Employee, Department:".len());
    println!("{underline}");
    for emp_dpt in all_employees.iter() {
        println!("{}, {}", emp_dpt.0, emp_dpt.1);
    }
}

/// **Print all unique employees in department and their department, for all employees in the company,
/// in alphabetical order by employee name**
///
/// If there are two or more employees of the same name in a department, only one will be shown.
/// Only sorts the printout, as HashMap cannot be sorted, at least not by using standard library.
fn print_all_unique_per_dept(employees: &HashMap<String, Vec<String>>) {
    let mut all_employees = BTreeSet::new();  // {(employee, dept)} <=> {(&String, &String)}
    for (dept, employees) in employees.iter() {
        for employee in employees {
            all_employees.insert((employee, dept));
        }
    }

    let heading = "Unique Employee, Department:";
    println!("{heading}");
    let underline = "=".repeat(heading.len());
    println!("{underline}");
    for emp_dpt in all_employees.iter() {
        println!("{}, {}", emp_dpt.0, emp_dpt.1);
    }
}

/// **Print all employees by departments sorted in alphabetical order both by dept (first) and then by employee name**
///
/// Only sorts the printout, as `HashMap` cannot be sorted, at least not by using standard library.
fn print_all_by_dept(employees: &HashMap<String, Vec<String>>) {
    let mut dept_employees = employees.iter().collect::<Vec<_>>();
    dept_employees.sort_unstable();
    for (dept, employees) in dept_employees.iter_mut() {
        let mut emp = employees.clone();
        emp.sort_unstable();
        println!("Department \"{dept}\":");
        for employee in emp.iter() {
            println!("\t{employee}");
        }
    }
}

/// Remove an employee from a department
fn remove_employee(words: &Vec<&str>, employees: &mut HashMap<String, Vec<String>>) {
    if (words.len() < 4) || !(words.contains(&"from")) {
        println!("The Remove command: {REMOVE} 'employee name' from 'dept name'");
        return;
    }

    let from_pos = words.iter().position(|&r| r == "from").
        expect("The REMOVE command must contain \"from\".");

    let name = words[1..from_pos].join(" ").trim_matches(|c| c == '\'' || c == '\"').trim().to_string();

    let binding = words[from_pos+1..].join(" ");
    let dept_name = binding.trim_matches(|c| c == '\'' || c == '\"').trim();

    if let Some(dept) = employees.get_mut(dept_name) {
        if let Some(pos) = dept.iter().position(|x| *x == name) {
            dept.swap_remove(pos);
        } else {
            println!("The employee \"{name}\" doesn't exist in the department \"{dept_name}\".");
        }
    } else {
        println!("The department \"{dept_name}\" doesn't exist.");
    }
}

/// Remove a department with all its employees
fn remove_dept(words: &Vec<&str>, employees: &mut HashMap<String, Vec<String>>) {
    if words.len() < 2 {
        println!("The Remove Department command: {REMOVE_DEPT} 'dept name'");
        return;
    }

    let dept = words[1..].join(" ").trim_matches(|c| c == '\'' || c == '\"').trim().to_string();

    if let Some(_) = employees.get_mut(&dept) {
        employees.remove(&dept);
    } else {
        println!("The department \"{dept}\" doesn't exist.");
    }
}

/// Clear the company
fn clear(employees: &mut HashMap<String, Vec<String>>) {
    let mut line = String::new();
    println!("Clear entire company? ['yes' to confirm]");
    stdin().read_line(&mut line).unwrap_or_default();
    if line.to_lowercase().trim() == "yes" {
        employees.clear();
        println!("Cleared the company.");
    }
}

fn main() {
    employees();
}
