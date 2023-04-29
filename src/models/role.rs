use crate::models::permission::Permission;
#[derive(Clone)]
#[derive(Debug)]
pub struct Role {
  pub id: String,
  pub value: String,
  pub name: String,
  pub permissions: Vec<Permission>
}