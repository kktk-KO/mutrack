use std::time::Instant;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result;
use std::io::Write;

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

impl MessageType {
    pub fn to_i32 (&self) -> i32 {
        match self {
            MessageType::Lock => 0,
            MessageType::UnLock => 1,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "message_type = {}, tid = {}", self.message_type, self.tid)
    }
}

impl Message {
    pub fn write<T>(&self, f: &mut T, t0: &Instant)
        where T: Write
    {
        let t1 = self.start_time.duration_since(*t0).as_nanos();
        let t2 = self.finish_time.duration_since(*t0).as_nanos();
        assert_eq!(f.write(&t1.to_be_bytes()).unwrap(), 16);
        assert_eq!(f.write(&t2.to_be_bytes()).unwrap(), 16);
        assert_eq!(f.write(&self.tid.to_be_bytes()).unwrap(), 4);
        assert_eq!(f.write(&self.message_type.to_i32().to_be_bytes()).unwrap(), 4);
        f.flush().unwrap();
    }
}
