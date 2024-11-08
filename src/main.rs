//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
//
//mark decker
//Nov 8 2024

//User will input both the Employee Name and the Department
//Department
//HashMap: key -> Name Value -> Department
//Vector of Names
//Vector of Departments
//
//HashMap where key is Department and Value is a vector of Strings?
//

use std::io;

struct Employee {
    name: String,
    department: String,
}

impl Employee {
    fn compare_department(&self, other: &Employee) -> bool {
        self.department == other.department
    }

}

fn build_employee(name: String, department: String) -> Employee {
    Employee {
        name,
        department,
    }
}

fn main() {

    let mut employees: Vec<Employee> = Vec::new();

    let mut stop: bool = false;
    
    while stop == false { //loop to allow multiple name, department to be input

        let mut department_in = String::new();
        let mut name_in = String::new();

        //put into a loop to enter many names and departments, use keyword to stop
        //
        println!("Please Enter the Employee Name:");
        io::stdin().read_line(&mut name_in).expect("Failed to read line");
        //need to strip off /n
        name_in.pop();

        if name_in.len() == 0 {
            stop = true;
            break
        }

        println!("Please Enter the Department:");
        io::stdin().read_line(&mut department_in).expect("Failed to read line");
        department_in.pop();  

        if name_in.len() == 0 {
            stop = true;
            break
        }

        println!("{} is in {department_in}",name_in);

        employees.push(build_employee(name_in,department_in));

    }

    for i in &employees {
        println!("{} -- {}",i.name,i.department);
    }

    //create a hashmap from the vector using department as key, assing names to a comma sperated
    //list

    //let s1 = String::from("Hello, ");
    //let s2 = String::from("world!");
    //let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used


}
