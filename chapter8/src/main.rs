use crate::services::company::Company;
use crate::services::company::Employee;
use std::collections::HashMap;
// use crate::services::hashmaps;
// use crate::services::strings;
// use crate::services::vectors;

pub mod services;

fn main() {
    // vectors::display_vectors();
    // strings::display_strings();
    // hashmaps::display_hash_maps();

    let employees: HashMap<String, Employee> = HashMap::new();

    let mut c = Company {
        name: "IBM".to_string(),
        employees,
    };

    c.add_employee("John", 30, 1000, "Manager");
    c.add_employee("Adam", 35, 10000, "CEO");
    c.add_employee("Jane", 21, 1500, "Accountant");
    c.display_employees();
}
