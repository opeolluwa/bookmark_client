pub struct Notification {
    pub r#type: String,
    pub message: String,
    pub title: String,
    pub action: NotificationAction,
}

pub enum NotificationAction {
    AcceptCollaborations,
    DeclineCollaboration,
}

impl Notification {
    pub fn new() -> Self {
        todo!()
    }
}
