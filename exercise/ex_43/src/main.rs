// Student Management System Let's consider a scenario where you need to develop a simple student management system. We can start by defining a struct to represent a student's information


struct Student {
    name:String,
    id:usize,
    grade:u8,
}


impl Student {
    fn new(name:String,id:usize,grade:u8) -> Self {
        Self {
            name,
            id,
            grade,
        }
    }

    fn display_info(&self) {
        println!("Student name is {}" , self.name);
        println!("Student id is {}" , self.id);
        println!("Student grade is {}" , self.grade);
    }
}


fn main() {

    let st1:Student = Student::new(String::from("Ali İhsan Tas"), 123 , 95);
    st1.display_info();
    
}
