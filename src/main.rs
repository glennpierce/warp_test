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

    let (tx, rx) = oneshot::channel::<()>();

    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
                 rx.await.ok();
    });

    // Spawn the server into a runtime
    let s = tokio::task::spawn(server);

    s.await;

    // Later, start the shutdown...
    //let _ = tx.send(());
}
