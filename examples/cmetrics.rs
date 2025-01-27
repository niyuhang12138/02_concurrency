use std::{thread, time::Duration};

use anyhow::Result;
use rand::Rng;
use template::CMapMetrics;
const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = CMapMetrics::new();

    // start  N workers and M requesters
    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }

    // Ok(())
}

fn task_worker(idx: usize, metrics: CMapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{idx}"))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_worker(metrics: CMapMetrics) -> Result<()> {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{page}")).unwrap();
    });

    Ok(())
}
