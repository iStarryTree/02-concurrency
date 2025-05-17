use anyhow::Result;
use concurrency::AmapMetrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "call.thread.worker.2",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);

    // start N workers and M Requests
    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_work(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do stuff
            let mut rng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_work(metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do stuff
            let mut rng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(50..800)));
            let page = rng.random_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
