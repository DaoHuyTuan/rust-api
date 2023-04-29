use crate::models::role::Role;
use crate::models::user::IUser;

#[derive(Clone)]
pub struct IOrganization {
    pub id: String,
    pub name: String,
    pub org_type: String,
    pub employee: Vec<IUser>,
    pub roles: Vec<Role>,
}

impl IOrganization {
    fn edit(id: String, org: IOrganization) {}
    fn delete(id: String) {}
    fn create_role(role: Role) {}
    fn edit_role(id: String, role: Role) {}
    fn delete_role(id: String) {}
    fn toggle_permission(permission_id: String, role_id: String) {}
}
