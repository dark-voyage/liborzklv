use teloxide::{payloads::*, prelude::*, requests::JsonRequest, types::*};

pub trait Topics {
    type Err: std::error::Error + Send;
    type SendMessageTF: Request<Payload = SendMessage, Err = Self::Err>;
    type ForwardMessageTF: Request<Payload = ForwardMessage, Err = Self::Err>;

    /// For Telegram documentation see [`SendMessage`].
    fn send_message_tf<C, T>(&self, chat_id: C, text: T, message: &Message) -> Self::SendMessageTF
    where
        C: Into<Recipient>,
        T: Into<String>;

    /// For Telegram documentation see [`ForwardMessage`].
    fn forward_message_tf<C, T>(
        &self,
        from_chat_id: C,
        to_chat_id: C,
        from_message_id: MessageId,
        thread: &ThreadId,
    ) -> Self::ForwardMessageTF
    where
        C: Into<Recipient>;
}

impl Topics for Bot {
    type Err = teloxide::errors::RequestError;
    type SendMessageTF = JsonRequest<teloxide::payloads::SendMessage>;
    type ForwardMessageTF = JsonRequest<teloxide::payloads::ForwardMessage>;

    fn send_message_tf<C, T>(&self, chat_id: C, text: T, message: &Message) -> Self::SendMessageTF
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        match message.thread_id {
            Some(thread_id) => Self::SendMessageTF::new(
                self.clone(),
                teloxide::payloads::SendMessage::new(chat_id, text),
            )
            .message_thread_id(thread_id),
            None => Self::SendMessageTF::new(
                self.clone(),
                teloxide::payloads::SendMessage::new(chat_id, text),
            ),
        }
    }

    fn forward_message_tf<C, T>(
        &self,
        from_chat_id: C,
        to_chat_id: C,
        from_message_id: MessageId,
        thread: &ThreadId,
    ) -> Self::ForwardMessageTF
    where
        C: Into<Recipient>,
    {
        match thread {
            ThreadId(MessageId(0)) => Self::ForwardMessageTF::new(
                self.clone(),
                teloxide::payloads::ForwardMessage::new(to_chat_id, from_chat_id, from_message_id),
            ),

            _ => Self::ForwardMessageTF::new(
                self.clone(),
                teloxide::payloads::ForwardMessage::new(to_chat_id, from_chat_id, from_message_id),
            )
            .message_thread_id(*thread),
        }
    }
}
