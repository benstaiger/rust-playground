use std::time::{Duration, Instant};

use futures::stream::{FuturesOrdered, FuturesUnordered, StreamExt};
use tokio::pin;
use tokio::time::sleep;

/// Adapted from C# example:
/// https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/concepts/async/start-multiple-async-tasks-and-process-them-as-they-complete?pivots=dotnet-6-0

async fn delay_and_return(val: u64) -> u64 {
    sleep(Duration::from_secs(val)).await;
    val
}

async fn read_ordered() {
    let mut stream = (1..=5)
        .map(|v| delay_and_return(v))
        .collect::<FuturesOrdered<_>>();

    let start = Instant::now();
    while let Some(v) = stream.next().await {
        println!("got {}. {:.2?} after start", v, Instant::now() - start);
    }
}

async fn process_url(url: &str) -> usize {
    let req = reqwest::get(url).await;
    match req {
        Ok(req) => {
            let b = req.bytes().await;
            b.map_or(0, |b| b.len())
        }
        Err(_) => 0,
    }
}

async fn sum_pages_serial(urls: &[&'static str]) -> usize {
    let now = Instant::now();

    let mut total = 0;
    for url in urls {
        let size = process_url(url).await;
        total += size;
    }

    let duration = now.elapsed();
    println!("Total bytes returned:  {}", total);
    println!("Elapsed time serially: {:?}", duration);
    total
}

async fn sum_pages_unordered(urls: &[&'static str]) -> usize {
    let now = Instant::now();

    // All tasks get started here.
    let downloads = urls
        .iter()
        .map(|url| tokio::spawn(process_url(url)))
        .collect::<FuturesUnordered<_>>();
    pin!(downloads);

    let mut total = 0;
    while let Some(res) = downloads.next().await {
        let size = res.unwrap();
        total += size;
    }

    let duration = now.elapsed();
    println!("Total bytes returned:   {}", total);
    println!("Elapsed time unordered: {:?}", duration);
    total
}

async fn read_and_sum() {
    let urls: Vec<&'static str> = vec![
        "https://docs.microsoft.com",
        "https://docs.microsoft.com/aspnet/core",
        "https://docs.microsoft.com/azure",
        "https://docs.microsoft.com/azure/devops",
        "https://docs.microsoft.com/dotnet",
        "https://docs.microsoft.com/dynamics365",
        "https://docs.microsoft.com/education",
        "https://docs.microsoft.com/enterprise-mobility-security",
        "https://docs.microsoft.com/gaming",
        "https://docs.microsoft.com/graph",
        "https://docs.microsoft.com/microsoft-365",
        "https://docs.microsoft.com/office",
        "https://docs.microsoft.com/powershell",
        "https://docs.microsoft.com/sql",
        "https://docs.microsoft.com/surface",
        "https://docs.microsoft.com/system-center",
        "https://docs.microsoft.com/visualstudio",
        "https://docs.microsoft.com/windows",
        "https://docs.microsoft.com/xamarin",
    ];

    sum_pages_unordered(&urls).await;
    sum_pages_serial(&urls).await;
}

#[tokio::main]
async fn main() {
    read_and_sum().await;
    read_ordered().await;
}
