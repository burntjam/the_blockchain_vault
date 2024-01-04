use std::thread;
use std::time::Duration;
use tokio::sync::oneshot;
use spool_server::spooler_impl::*;

use spool_client::spool_connection_grpc::*;


#[tokio::test]
async fn test_spool_connection_grpc_one() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let spooler_impl = spool_server::spooler_impl::SpoolerService::new();

    rt.spawn(async move{
        let _ = spooler_impl.run().await.unwrap();
    });
    
    thread::sleep(Duration::from_millis(1000));
    println!("Before creating the connection");
    let spool_client = 
        spool_client::spool_connection_grpc::createGrpcConnection("http://0.0.0.0:50080".to_string(), "test".to_string()).await.unwrap();
    
    spool_client.push(vec![1,2,3,4,5,6]).await.unwrap();
    spool_client.push(vec![2,2,2,2,2,2]).await.unwrap();
    spool_client.pushToTopic(vec![2,2,2,2,2,2],&"test".to_string()).await.unwrap();

    let result = spool_client.consume().await.unwrap();
    assert_eq!(result.len(), 3);

    spool_client.push(vec![1,2,3,4,5,6]).await.unwrap();
    spool_client.pushToTopic(vec![2,2,2,2,2,2],&"test".to_string()).await.unwrap();

    let result2 = spool_client.consumeFromTopic(&"test".to_string()).await.unwrap();
    assert_eq!(result2.len(), 2);


    // Trigger the shutdown
    //let _ = tx.send(());
    rt.shutdown_background();

}