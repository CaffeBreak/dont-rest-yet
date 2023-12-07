use crate::misc::id::Id;

use super::user::User;

#[derive(Debug, Clone)]
pub(crate) struct Group {
    pub(crate) id: Id,
    pub(crate) name: String,
    pub(crate) users: Vec<User>,
}
