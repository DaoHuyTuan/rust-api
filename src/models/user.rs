use crate::models::role::Role;
#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub org_id: String,
    pub role: Vec<Role>,
}

impl User {}
