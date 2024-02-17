use teloxide::payloads;
use teloxide::prelude::*;
use teloxide::requests::JsonRequest;
use teloxide::types::{MessageId, Recipient};

#[async_trait::async_trait]
pub trait Timer {
    type Err: std::error::Error + Send;
    type DeleteTimer: Request<Payload = payloads::DeleteMessage, Err = Self::Err>;

    async fn delete_timer<C>(&self, chat_id: C, message_id: MessageId, timer: usize) -> Self::DeleteTimer
    where
        C: Into<Recipient> + Send + Sync;
}

impl Timer for Bot {
    type Err = teloxide::errors::RequestError;
    type DeleteTimer = JsonRequest<teloxide::payloads::DeleteMessage>;

    async fn delete_timer<C>(&self, chat_id: C, message_id: MessageId, timer: usize) -> Self::DeleteTimer
    where
        C: Into<Recipient> + Send + Sync,
    {
        tokio::time::sleep(tokio::time::Duration::from_secs(timer as u64)).await;

        Self::DeleteTimer::new(self.clone(), payloads::DeleteMessage::new(chat_id, message_id))

    }
}