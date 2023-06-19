extern crate tokio;
use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
pub async fn main() {
    let handle = tokio::spawn(count_to(10));

    for i in 1..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }
    let _res = handle.await;
    count_to(10).await;
}
