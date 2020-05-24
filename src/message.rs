use std::time::Instant;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result;

pub enum MessageType {
    Lock,
    UnLock,
}

pub struct Message {
    pub start_time: Instant,
    pub finish_time: Instant,
    pub tid: i32,
    pub message_type: MessageType,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MessageType::Lock => write!(f, "Lock"),
            MessageType::UnLock=> write!(f, "UnLock"),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "message_type = {}, tid = {}", self.message_type, self.tid)
    }
}
