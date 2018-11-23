use std::collections::HashMap;

pub struct CompanyEmployees {
    dep_employee_map: HashMap<String, Vec<String>>,
}

impl CompanyEmployees {
    pub fn new() -> CompanyEmployees {
        CompanyEmployees {
            dep_employee_map: HashMap::new()
        }
    }

    pub fn add_entry(&mut self, input: String) {
        let splits = input.split(' ');
        let tokens: Vec<&str> = splits.collect();

        let entry = self.dep_employee_map.entry(tokens[3].to_string())
                                         .or_insert(Vec::new());

        entry.push(tokens[1].to_string());
        entry.sort_unstable();
    }

    pub fn view_all(&self) {
        println!("All: {:?}", self.dep_employee_map);
    }

    pub fn get_employees(&self, dep_name: &str) -> Option<&Vec<String>> {
        self.dep_employee_map.get(&dep_name.to_string())
    }
}
