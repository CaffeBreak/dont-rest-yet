use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{driver::grpc_api::reminder, misc::error::ReminderError};

use super::group::Group;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserIdentifier {
    pub(crate) client: String,
    pub(crate) identifier: String,
}
impl From<reminder::UserIdentifier> for UserIdentifier {
    fn from(value: reminder::UserIdentifier) -> Self {
        Self {
            client: value.client,
            identifier: value.identifier,
        }
    }
}
impl Into<reminder::UserIdentifier> for UserIdentifier {
    fn into(self) -> reminder::UserIdentifier {
        reminder::UserIdentifier {
            client: self.client,
            identifier: self.identifier,
        }
    }
}
impl Display for UserIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*{}", self.client, self.identifier)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct User {
    pub(crate) user_identifier: UserIdentifier,
    pub(crate) groups: Vec<Group>,
}
impl Into<reminder::User> for User {
    fn into(self) -> reminder::User {
        reminder::User {
            id: Some(self.user_identifier.into()),
            group_id: self
                .groups
                .into_iter()
                .map(|group| group.id.to_string())
                .collect(),
        }
    }
}

pub(crate) trait UserRepository {
    async fn create(&self, id: UserIdentifier) -> Result<User, ReminderError>;
    async fn get(&self, id: UserIdentifier) -> Result<User, ReminderError>;
    async fn list(&self, group: Option<Group>) -> Result<Vec<User>, ReminderError>;
    async fn delete(&self, id: UserIdentifier) -> Result<User, ReminderError>;
}
