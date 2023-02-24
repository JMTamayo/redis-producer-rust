use std::collections::BTreeMap;
use redis::{Commands, RedisResult};

extern crate redis;

pub struct RedisProducer {
    pub redis_host: String,
}

impl RedisProducer {

    async fn get_redis_connection(self) -> redis::Connection {

        let host = format!("redis://{}",self.redis_host);

        let client = redis::Client::open(host).expect("Redis Client failed");
        let conn = client.get_connection().expect("Redis connection failed");
    
        return conn;
    
    }

    pub async fn produce(self, stream_name: String, map: BTreeMap<String, String>) -> RedisResult<String>{

        let mut conn = self.get_redis_connection().await;

        let key = stream_name.as_str();
        let id = "*";

        return conn.xadd_map(key, id, map);

    }

}