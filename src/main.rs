use redis_producer_rust::config::config::get_settings;
use redis_producer_rust::utils::redis::RedisProducer;
use redis_producer_rust::utils::utils::{get_message, map_message};

#[tokio::main]
async fn main() {

    let settings = get_settings();

    let msg = get_message();
    let map = map_message(msg);

    let producer = RedisProducer {
        redis_host: settings.host,
    };
    let result = producer.produce(settings.stream_name, map).await;

    match result {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e)
    }
    
}