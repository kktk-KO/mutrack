use std::thread::spawn;
use std::thread::JoinHandle;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::sync_channel;
use std::fs::File;
use std::io::BufWriter;
use lazy_static::lazy_static;
use crate::message::Message;
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

    let mut f = BufWriter::new(File::create("mutrack.data").unwrap());

    message.write(&mut f, &t0);
    loop {
        let message = receiver.recv().unwrap();
        message.write(&mut f, &t0);
    }
}

