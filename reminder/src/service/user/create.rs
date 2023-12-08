use crate::{
    domain::user::{User, UserIdentifier, UserRepository},
    misc::error::ReminderError,
    service::service::UserService,
};

impl<T: UserRepository> UserService<T> {
    pub(crate) async fn create_user(&self, id: UserIdentifier) -> Result<User, ReminderError> {
        let create_result = self.user_repo.create(id).await;

        create_result
    }
}
