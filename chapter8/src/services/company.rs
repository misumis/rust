use std::collections::HashMap;

pub struct Employee {
    name: String,
    age: u8,
    salary: u32,
}

impl Employee {
    fn new(name: &str, age: &u8, salary: &u32) -> Employee {
        Employee {
            name: (*name).to_string(),
            age: *age,
            salary: *salary,
        }
    }
}

pub struct Company {
    pub name: String,
    pub employees: HashMap<String, Employee>,
}

impl Company {
    pub fn add_employee(&mut self, name: &str, age: u8, salary: u32, position: &str) {
        let employee = Employee::new(name, &age, &salary);
        self.employees.insert(position.to_string(), employee);
    }
    pub fn display_employees(&self) {
        for (position, employee) in &self.employees {
            println!(
                "{}: {} - {}$ Age: {}",
                position, employee.name, employee.salary, employee.age
            );
        }
    }
}
