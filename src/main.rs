use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;
const DATA: [u8; 4] = [1, 2, 3, 4];

const OPTIONS: ping_rs::PingOptions = ping_rs::PingOptions {
    ttl: 128,
    dont_fragment: true,
};

#[derive(Parser)]
struct Config {
    #[arg(short, long, help = "Verbose output")]
    verbose: bool,
    #[arg(
        long,
        short,
        help = "Quite mode, omits errors, and just output the results"
    )]
    quite: bool,

    #[arg(short, long, help = "Timeout", default_value = "5")]
    timeout: u64,

    #[arg(short, help = "Mark live/down hosts with +/-", default_value = "false")]
    u: bool,

    #[arg(long, short, help = "The number of retries", default_value = "1")]
    retry: u64,
}
#[tokio::main]
async fn main() {
    let config = Config::parse();
    let mut tasks = vec![];
    let live_count = Arc::new(Mutex::new(0usize));
    let down_count = Arc::new(Mutex::new(0usize));
    let inputs = std::io::stdin().lines().filter_map(|line| line.ok());
    for input in inputs {
        let input = format!("{}:{}", input, 1234);
        let (live_count, down_count) = (Arc::clone(&live_count), Arc::clone(&down_count));
        let task = tokio::spawn(async move {
            let mut failed = true;
            for i in 0..config.retry {
                match tokio::net::lookup_host(&input).await {
                    Ok(mut addresses) => {
                        let input = &input[..(input.len() - 5)]; // slice off the port part
                        if let Some(address) = addresses.next() {
                            let arc = std::sync::Arc::new(&DATA[..]);
                            let res = ping_rs::send_ping_async(
                                &address.ip(),
                                std::time::Duration::from_secs(config.timeout),
                                arc,
                                Some(&OPTIONS),
                            )
                            .await;
                            match res {
                                Ok(_) => {
                                    failed = false;
                                    let mut live_count = live_count.lock().await;
                                    *live_count += 1;
                                    if config.u {
                                        println!("+ {}", input);
                                    } else {
                                        println!("{}", input);
                                    }
                                    break;
                                }
                                Err(e) => {
                                    if !config.quite {
                                        eprintln!("\tError: {:?}", e)
                                    }
                                }
                            }
                        } else {
                            if !config.quite {
                                eprintln!("\tFailed to resolve {}", input)
                            };
                        }
                    }
                    Err(e) => {
                        if !config.quite {
                            eprintln!("\tError: {}", e.to_string())
                        }
                    }
                }
                if !config.quite && i < config.retry - 1 {
                    eprintln!("Retrying {}", input);
                }
            }
            if failed && config.u {
                println!("- {}", &input[..input.len() - 5]);
            }
            if failed {
                let mut down_count = down_count.lock().await;
                *down_count += 1;
            }
        });
        tasks.push(task);
    }
    for task in tasks {
        task.await.unwrap();
    }

    if config.verbose {
        let (live_count, down_count) = (live_count.lock().await, down_count.lock().await);
        println!("Live count: {}\nDown count: {}", live_count, down_count);
    }
}
