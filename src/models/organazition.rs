use crate::models::role::Role;
use crate::models::user::User;

#[derive(Clone)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub org_type: String,
    pub employee: Vec<User>,
    pub roles: Vec<Role>,
}

impl Organization {
    fn create(org: Organization) {}
    fn edit(id: String, org: Organization) {}
    fn delete(id: String) {}
    fn create_role(role: Role) {}
    fn edit_role(id: String, role: Role) {}
    fn delete_role(id: String) {}
    fn toggle_permission(permission_id: String, role_id: String) {}
}
