use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::domain::group::Group;

use super::user_repository::UserRecord;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct GroupRecord {
    pub(super) id: Thing,
    pub(super) name: String,
    pub(super) users: Vec<Thing>,
}
impl From<Group> for GroupRecord {
    fn from(value: Group) -> Self {
        Self {
            id: Thing::from(("group".to_string(), value.id.to_string())),
            name: value.name,
            users: value
                .users
                .iter()
                .map(|user| Thing::from(("user".to_string(), user.user_identifier.to_string())))
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GroupRecordWithUser {
    id: Thing,
    name: String,
    users: Vec<UserRecord>,
}
impl From<Group> for GroupRecordWithUser {
    fn from(value: Group) -> Self {
        Self {
            id: Thing::from(("group".to_string(), value.id.to_string())),
            name: value.name,
            users: value.users.into_iter().map(|user| user.into()).collect(),
        }
    }
}
