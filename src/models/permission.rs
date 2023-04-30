#[derive(Clone, Debug)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub value: String,
}

impl Permission {
    fn toggle_super_admin(toggle: bool) -> bool {
        return true;
    }
    fn toggle_create_role(toggle: bool) -> bool {
        return true;
    }
    fn toggle_edit_role(toggle: bool) -> bool {
        return true;
    }
    fn toggle_edit_user(toggle: bool) -> bool {
        return true;
    }
}
