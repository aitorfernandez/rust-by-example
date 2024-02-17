use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    // Create a new channel with a capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx, // is the sender to send later the response
        };
        // Send the GET request
        tx.send(cmd).await.unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {res:?}");

        // let res = client.get("foo").await;
    });
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };
        tx2.send(cmd).await.unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {res:?}");

        // client.set("foo", "bar".into()).await;
    });

    // The manager task opens a connection to the mini-redis server and processes commands received on the channel.
    let manager = tokio::spawn(async move {
        // Creates the connection with the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    // Send the response over the oneshot channel.
                    // send in oneshot completes immediately and does not require an .await.
                    // This is because send on a oneshot channel will always fail or succeed immediately without any form of waiting.
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();

    manager.await.unwrap();
}
