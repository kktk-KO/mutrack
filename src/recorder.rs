use std::thread::spawn;
use std::thread::JoinHandle;
use lazy_static::lazy_static;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::sync_channel;
use crate::message::Message;
use crate::debug::print_debug;
use crate::activate::activate;

struct Recorder {
    worker: JoinHandle<()>,
    sender: SyncSender<Message>,
}

lazy_static! {
    static ref RECORDER: Recorder = {
        let (sender, receiver) = sync_channel(1024);
        Recorder{ worker: spawn(move || { worker(receiver) }), sender: sender}
    };
}

pub fn ensure_recorder() {
    RECORDER.worker.thread(); // touch RECORDER to call constructor
}

pub fn record(message: Message) {
    RECORDER.sender.send(message).unwrap();
}

fn worker(receiver: Receiver<Message>) {
    activate();
    let message = receiver.recv().unwrap();
    // TODO: is there such guarantee that the first message is generated first???
    let t0 = message.start_time;
    print_debug(format!("{} start_time = {} finish_time = {} \n",
        message,
        message.start_time.duration_since(t0).as_nanos(),
        message.finish_time.duration_since(t0).as_nanos(),
    ));

    loop {
        let message = receiver.recv().unwrap();
        print_debug(format!("{} start_time = {} finish_time = {} \n",
            message,
            message.start_time.duration_since(t0).as_nanos(),
            message.finish_time.duration_since(t0).as_nanos(),
        ));
    }
}
