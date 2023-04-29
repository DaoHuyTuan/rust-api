use crate::models::role::Role;
#[derive(Clone)]
pub struct IUser {
  pub id: String,
  pub name: String,
  pub org_id: String,
  pub role: Vec<Role>
}

impl IUser {
}