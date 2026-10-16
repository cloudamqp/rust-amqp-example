use std::time::Duration;

use amqp_client_rs::QueueDeclareOpts;
use rust_amqp_example::{connect, consume, publish};

#[tokio::test]
async fn worker_receives_published_message() {
    let queue = format!("roundtrip-{}", std::process::id());
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // A connection of its own for consuming, so that publishing never blocks acks
    let consumer = connect().await.unwrap();
    let consumer_queue = queue.clone();
    tokio::spawn(async move {
        consume(&consumer, &consumer_queue, move |message| {
            tx.send(message).unwrap();
        })
        .await
    });

    let publisher = connect().await.unwrap();
    publish(&publisher, &queue, "Hello CloudAMQP!")
        .await
        .unwrap();

    let received = tokio::time::timeout(Duration::from_secs(5), rx.recv()).await;
    assert_eq!(received.unwrap().as_deref(), Some("Hello CloudAMQP!"));

    let opts = QueueDeclareOpts {
        durable: true,
        ..Default::default()
    };
    let q = publisher.queue(&queue, opts).await.unwrap();
    q.delete(Default::default()).await.unwrap();
}
