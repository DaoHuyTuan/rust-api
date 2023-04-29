// use crate::apis::user::User;


use crate::models::organazition::IOrganization;
use crate::models::permission::Permission;
use crate::models::role::Role;
use crate::models::user::IUser;

pub mod models;
pub mod utils;
fn main() {

    let admin = Permission {
        id: "admin".to_string(),
        name: "admin".to_string(),
        value: "admin".to_string(),
    };

    let first_role = Role {
        id: "1".to_string(),
        name: "admin".to_string(),
        value: "admin".to_string(),
        permissions: vec![admin],
    };
    let user = IUser {
        id: "132".to_string(),
        name: "zyye".to_string(),
        org_id: "gfrfref".to_string(),
        role: vec![first_role],
    };
    let google = IOrganization {
        id: "132google".to_string(),
        name: "google".to_string(),
        org_type: "free".to_string(),
        employee: vec![user],
        roles: vec![first_role.clone()],
    };
    for role in &google.roles {
        println!("Role: {:#?}", role);
    }

}
