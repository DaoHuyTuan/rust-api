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
}
