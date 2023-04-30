// use crate::apis::user::User;

use crate::models::organazition::Organization;
use crate::models::permission::Permission;
use crate::models::role::Role;
use crate::models::user::User;

pub mod models;
pub mod utils;
fn main() {
    let admin = Permission {
        id: "admin".to_string(),
        name: "admin".to_string(),
        value: "admin".to_string(),
    };
    let account: Permission = Permission {
        id: "account".to_string(),
        name: "account".to_string(),
        value: "account".to_string(),
    };
    let first_role = Role {
        id: "1".to_string(),
        name: "admin".to_string(),
        value: "admin".to_string(),
        permissions: vec![admin],
    };
    let second_role = Role {
        id: "1".to_string(),
        name: "admin".to_string(),
        value: "admin".to_string(),
        permissions: vec![account],
    };
    let roles = vec![first_role.clone(), second_role.clone()];
    let user = User {
        id: "132".to_string(),
        name: "zyye".to_string(),
        org_id: "gfrfref".to_string(),
        role: vec![first_role],
    };

    let google: Organization = Organization {
        id: "132google".to_string(),
        name: "google".to_string(),
        org_type: "free".to_string(),
        employee: vec![user],
        roles: roles,
    };
    println!("Roles: {:#?}", google.employee);
}
