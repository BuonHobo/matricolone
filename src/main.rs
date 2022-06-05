mod students;
use students::io::*;

fn main() {
    let mut students = get_students();
    students.sort_by(|a, b| b.cmp(a));
    if store_students(&students).is_err() {
        eprintln!("Something went wrong")
    };
}
