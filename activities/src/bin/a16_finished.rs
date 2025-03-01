// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Locker {
    student_name: String,
    locker_assignment: Option<i32>,
}

fn main() {
    let student = Locker {
        student_name: "Carlee".to_owned(),
        locker_assignment: Some(142),
    };

    println!("student: {:?}", student.student_name);

    match student.locker_assignment {
        Some(ans)=> println!("locker assignment: {:?}", ans),
        None => println!("no locker assigned"),
    }
}

