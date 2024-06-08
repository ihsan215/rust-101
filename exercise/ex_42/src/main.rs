//  Write a Rust program that defines a struct Employee with fields id, name, and salary. Now write a function to calculate an employee's annual salary.

struct Employee {
    id:u32,
    name:String,
    salary:u32,
}


impl Employee {
    fn calculate_annual_salary(&self) -> usize {
        self.salary as usize * 12 as usize
    }
}

fn main() { 

    let employee: Employee = Employee {
        id:123,
        name:String::from("ali ihsan"),
        salary: 4000,
    };

    println!("ID: {} " , employee.id);
    println!("Name: {} " , employee.name);
    println!("Salary: {} " , employee.salary);
    println!("Annual Salary: {} " , employee.calculate_annual_salary());

}