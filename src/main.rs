use crate::apis::user::User;

mod apis {
    pub mod user;
}

pub mod utils;
fn main() {
    User::hello();
    println!("Hello, world!");
}
