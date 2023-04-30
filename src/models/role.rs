use crate::models::permission::Permission;
#[derive(Clone, Debug)]
pub struct Role {
    pub id: String,
    pub value: String,
    pub name: String,
    pub permissions: Vec<Permission>,
}

impl Role {
    pub fn create(role: Role) {}
}
