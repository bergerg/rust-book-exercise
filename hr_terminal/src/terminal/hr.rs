use super::common::input;

#[derive(Debug)]
pub enum TermialInput {
    Exit,
    CreateDepartment,
    ShowDepartments,
    Hire,
}

pub fn hr_term() -> TermialInput {

    let menu = "
What would you like to do?
1. Create a new Department
2. Show all departments
3. Hire a new employee
4. Exit
";

    let user_choise: TermialInput;

    loop {
    
        user_choise = match input(menu).trim().parse() {
            Ok(num) => match num {
                1 => TermialInput::CreateDepartment,
                2 => TermialInput::ShowDepartments,
                3 => TermialInput::Hire,
                4 => TermialInput::Exit,
                _ => {
                    println!("Invalid option.");
                    continue
                },
            },
            Err(_) => {
                println!("Invalid option.");
                continue
            },
        };
        break;
    };
    user_choise
}