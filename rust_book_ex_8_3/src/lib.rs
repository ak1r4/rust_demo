pub mod vector_ex;
pub mod string_ex;
pub mod hashmap_ex;

#[cfg(test)]
mod tests {
    use super::vector_ex;
    use super::string_ex;
    use super::hashmap_ex;

    #[test]
    fn test_vector_exercise() {
        assert_eq!(vector_ex::get_mean(vec![1, 2, 3, 4, 5]), 3.0);
        assert_eq!(vector_ex::get_mean(vec![1, 2, 5]), 8. / 3.);
        assert_eq!(vector_ex::get_mean(vec![1]), 1.);

        assert_eq!(vector_ex::get_median(vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(vector_ex::get_median(vec![1, 2, 4, 5]), 4);
        assert_eq!(vector_ex::get_median(vec![1]), 1);

        assert_eq!(vector_ex::get_mode(vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(vector_ex::get_mode(vec![1, 5, 2, 3, 5]), 5);
        assert_eq!(vector_ex::get_mode(vec![1]), 1);
    }

    #[test]
    fn test_string_exercise() {
        assert_eq!(string_ex::get_pig_latin(String::from("apple")), String::from("apple-hay"));
        assert_eq!(string_ex::get_pig_latin(String::from("APPLE")), String::from("APPLE-hay"));
        assert_eq!(string_ex::get_pig_latin(String::from("first")), String::from("irst-fay"));
        assert_eq!(string_ex::get_pig_latin(String::from("FIRST")), String::from("IRST-Fay"));
    }

    #[test]
    fn test_hashmap_exercise() {
        let mut company_employees = hashmap_ex::CompanyEmployees::new();

        company_employees.add_entry("Add Sally to Engineering".to_string());
        company_employees.add_entry("Add Lumo to Sales".to_string());
        company_employees.add_entry("Add Amir to Sales".to_string());
        company_employees.add_entry("Add Celo to Sales".to_string());
        company_employees.add_entry("Add Holo to Sales".to_string());

        assert_eq!(company_employees.get_employees("Sales"),
                   Some(&vec![String::from("Amir"),
                              String::from("Celo"),
                              String::from("Holo"),
                              String::from("Lumo")]));

        assert_eq!(company_employees.get_employees("Engineering"),
                   Some(&vec![String::from("Sally")]));
    }
}
