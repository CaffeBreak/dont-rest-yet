use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{
    domain::{
        group::Group,
        user::{User, UserIdentifier, UserRepository},
    },
    init::DB,
    log,
    misc::{error::ReminderError, id::Id},
};

use super::group_repository::GroupRecord;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct UserRecord {
    pub(super) id: Thing,
    pub(super) groups: Vec<Thing>,
}
impl From<User> for UserRecord {
    fn from(value: User) -> Self {
        Self {
            id: Thing::from((
                "user".to_string(),
                surrealdb::sql::Id::from(value.user_identifier),
            )),
            groups: value
                .groups
                .iter()
                .map(|group| Thing::from(("group".to_string(), group.id.to_string())))
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct UserRecordWithGroup {
    pub(super) id: Thing,
    groups: Vec<GroupRecord>,
}
impl Into<User> for UserRecordWithGroup {
    fn into(self) -> User {
        User {
            user_identifier: self.id.id.try_into().unwrap(),
            groups: self
                .groups
                .into_iter()
                .map(|group| Group {
                    id: Id::from(group.id.id.to_string()),
                    name: group.name,
                    users: vec![],
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct CreateUserRecord {
    pub(super) id: Thing,
    groups: Vec<Thing>,
}

impl From<UserIdentifier> for surrealdb::sql::Id {
    fn from(value: UserIdentifier) -> Self {
        let map: HashMap<&str, surrealdb::sql::Value> = vec![
            ("client", value.client.into()),
            ("identifier", value.identifier.into()),
        ]
        .into_iter()
        .collect();
        Self::Object(surrealdb::sql::Object::from(map))
    }
}
impl TryInto<UserIdentifier> for surrealdb::sql::Id {
    type Error = ReminderError;

    fn try_into(self) -> Result<UserIdentifier, Self::Error> {
        if let surrealdb::sql::Id::Object(map) = self {
            let maybe_client = map.0.get("client");
            let maybe_identifier = map.0.get("identifier");
            match (maybe_client, maybe_identifier) {
                (Some(client), Some(identifier)) => Ok(UserIdentifier {
                    client: client.to_string().replace("'", ""),
                    identifier: identifier.to_string().replace("'", ""),
                }),
                _ => Err(ReminderError::UserIdToUserIdentifierFailed {
                    cause: "Client or identifier is not found.".to_string(),
                }),
            }
        } else {
            Err(ReminderError::UserIdToUserIdentifierFailed {
                cause: "not object type".to_string(),
            })
        }
    }
}

pub(crate) struct UserRepositorySurrealDriver;

impl UserRepository for UserRepositorySurrealDriver {
    async fn create(&self, id: UserIdentifier) -> Result<User, ReminderError> {
        let created: UserRecordWithGroup = DB
            .create(("user", surrealdb::sql::Id::from(id.clone())))
            .content(CreateUserRecord {
                id: Thing::from(("user", surrealdb::sql::Id::from(id))),
                groups: vec![],
            })
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .unwrap();
        log!("DEBUG" -> format!("Created: {:?}", created).dimmed());

        Ok(created.into())
    }

    async fn get(&self, id: UserIdentifier) -> Result<User, ReminderError> {
        let query = format!(
            "select * from {} fetch groups",
            Thing::from((
                "user".to_string(),
                Into::<surrealdb::sql::Id>::into(id.clone())
            ))
        );
        let user: Option<UserRecordWithGroup> = DB
            .query(query)
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .take(0)
            .map_err(|error| ReminderError::DBOperationError(error))?;
        let user = user.ok_or(ReminderError::UserNotFound { id: id.to_string() })?;
        log!("DEBUG" -> format!("Got: {:?}", user).dimmed());

        Ok(user.into())
    }

    async fn list(&self, group: Option<Group>) -> Result<Vec<User>, ReminderError> {
        let query = "select * from user".to_string();
        let query = match group {
            Some(group) => format!("{} where group == {}", query, group.id.to_string()),
            None => query,
        };
        let query = format!("{} fetch group", query);

        let list: Vec<UserRecordWithGroup> = DB
            .query(query)
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .take(0)
            .map_err(|error| ReminderError::DBOperationError(error))?;
        log!("DEBUG" -> format!("Listed: {:?}", list).dimmed());

        Ok(list.into_iter().map(|user| user.into()).collect())
    }

    async fn delete(&self, id: UserIdentifier) -> Result<User, ReminderError> {
        let deleted: UserRecordWithGroup = DB
            .delete(("user", Into::<surrealdb::sql::Id>::into(id)))
            .await
            .map_err(|error| ReminderError::DBOperationError(error))?
            .unwrap();
        log!("DEBUG" -> format!("Deleted: {:?}", deleted).dimmed());

        Ok(deleted.into())
    }
}
