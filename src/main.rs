use tokio::time::{sleep, Duration};

async fn add(left: i32, right: i32) -> i32 {
    sleep(Duration::from_millis(5000)).await;
    return left + right;
}

#[tokio::main]
async fn main() {
    let ans = add(10, 20).await;
    println!("{}", ans);
}
