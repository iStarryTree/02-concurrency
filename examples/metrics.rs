use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    // start N workers and M Requests
    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_work(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, mut metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do stuff
            let mut rng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(100..5000)));
            metrics.inc(format!("rcall.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_work(mut metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do stuff
            let mut rng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(50..800)));
            let page = rng.random_range(1..256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
