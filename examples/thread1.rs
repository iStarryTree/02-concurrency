use anyhow::Result;
use anyhow::anyhow;
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: i32,
}

fn main() -> Result<()> {
    // println!("Hello, world!");
    let (tx, rx) = mpsc::channel();

    // 创建producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    // 创建consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow!("consumer join error: {:?}", e))?;

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<i32>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
    }
}

impl Msg {
    fn new(idx: usize, value: i32) -> Self {
        Self { idx, value }
    }
}
