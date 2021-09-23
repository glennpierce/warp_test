use warp::Filter;
use futures::future::TryFutureExt;
use tokio::sync::oneshot;
use tokio::time::Duration;

async fn do_long_runing_thing() {
    loop {

        println!("Hi");
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn main() {

    let routes = warp::any()
    .map(|| "Hello, World!");

    // let routes_clone
    let handle = tokio::task::spawn(async move { 
        warp::serve(routes.clone()).run(([127, 0, 0, 1], 3030)).await
    });

    do_long_runing_thing().await;
}
