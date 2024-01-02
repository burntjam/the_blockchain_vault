use std::thread;
use std::time::Duration;
use spool_server::spooler_impl::*;

use spool_client::spool_connection_grpc::*;


#[test]
fn test_spool_connection_grpc_one() {
    // You need to create the runtime first
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Then you can use `block_on` to run your async code
    let future = rt.spawn(async {
        let spooler_impl = spool_server::spooler_impl::SpoolerService::new();

        let _ = spooler_impl.run().await;
    });
    
    thread::sleep(Duration::from_millis(1000));

    let spool_client = 
        spool_client::spool_connection_grpc::createGrpcConnection("http://0.0.0.0:50080".to_string(), "test".to_string()).unwrap();
    
    spool_client.push(vec![1,2,3,4,5,6]).unwrap();
    spool_client.push(vec![2,2,2,2,2,2]).unwrap();
    spool_client.pushToTopic(vec![2,2,2,2,2,2],&"test".to_string()).unwrap();

    let result = spool_client.consume().unwrap();
    assert_eq!(result.len(), 3);

    spool_client.push(vec![1,2,3,4,5,6]).unwrap();
    spool_client.pushToTopic(vec![2,2,2,2,2,2],&"test".to_string()).unwrap();

    let result2 = spool_client.consumeFromTopic(&"test".to_string()).unwrap();
    assert_eq!(result2.len(), 2);


    future.abort();

}