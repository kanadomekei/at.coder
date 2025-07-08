use std::collections::HashMap;

fn main() {
    println!("=== Rust実践的なサンプル ===");

    // 単語カウンター
    let text = "hello world wonderful world";
    let word_count = count_words(text);
    println!("単語カウント: {:?}", word_count);

    // 温度変換
    let celsius = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C = {}°F", celsius, fahrenheit);

    // フィボナッチ数列
    for i in 0..10 {
        println!("fibonacci({}) = {}", i, fibonacci(i));
    }

    // 従業員管理システム
    let mut company = Company::new();
    company.add_employee("Alice".to_string(), "Engineering".to_string());
    company.add_employee("Bob".to_string(), "Sales".to_string());
    company.add_employee("Charlie".to_string(), "Engineering".to_string());

    company.list_employees_by_department("Engineering");
    company.list_all_employees();
}

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    word_count
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[derive(Debug)]
struct Company {
    employees: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            employees: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: String, department: String) {
        let dept_employees = self.employees.entry(department).or_insert(Vec::new());
        dept_employees.push(name);
    }

    fn list_employees_by_department(&self, department: &str) {
        match self.employees.get(department) {
            Some(employees) => {
                println!("{}部門の従業員:", department);
                for employee in employees {
                    println!("  - {}", employee);
                }
            }
            None => println!("{}部門は存在しません", department),
        }
    }

    fn list_all_employees(&self) {
        println!("全従業員:");
        for (department, employees) in &self.employees {
            println!("{}部門:", department);
            for employee in employees {
                println!("  - {}", employee);
            }
        }
    }
}
