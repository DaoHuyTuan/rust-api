use crate::models::role::Role;
#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub org_id: String,
    pub role: Vec<Role>,
}

impl User {
    pub fn create(user: User) {}
    pub fn edit(user: User) {}
    pub fn delete(id: String) {}
    pub fn assign_role_to_user(user_id: String, role_id: String) {}
    pub fn assign_permission_to_role
}
