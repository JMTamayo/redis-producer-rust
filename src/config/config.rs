use std::env;

pub struct Settings {
    pub host: String,
    pub stream_name: String,
}

pub fn get_settings() -> Settings {

    let host = env::var("REDIS_HOST").expect("REDIS_HOST not found");
    let stream_name = env::var("REDIS_STREAM_NAME").expect("REDIS_STREAM_NAME not found");

    let settings = Settings {
        host: host,
        stream_name: stream_name,
    };

    return settings;
}

