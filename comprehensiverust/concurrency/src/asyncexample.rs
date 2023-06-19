use futures::executor::block_on;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count is: {i}!");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

pub fn main() {
    block_on(async_main(10));
    let future = async_main(10);
    block_on(future);
}
