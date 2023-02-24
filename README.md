```

██████╗░███████╗██████╗░██╗░██████╗     
██╔══██╗██╔════╝██╔══██╗██║██╔════╝     
██████╔╝█████╗░░██║░░██║██║╚█████╗░     
██╔══██╗██╔══╝░░██║░░██║██║░╚═══██╗     
██║░░██║███████╗██████╔╝██║██████╔╝     
╚═╝░░╚═╝╚══════╝╚═════╝░╚═╝╚═════╝░     

██████╗░██████╗░░█████╗░██████╗░██╗░░░██╗░█████╗░███████╗██████╗░   
██╔══██╗██╔══██╗██╔══██╗██╔══██╗██║░░░██║██╔══██╗██╔════╝██╔══██╗   
██████╔╝██████╔╝██║░░██║██║░░██║██║░░░██║██║░░╚═╝█████╗░░██████╔╝   
██╔═══╝░██╔══██╗██║░░██║██║░░██║██║░░░██║██║░░██╗██╔══╝░░██╔══██╗   
██║░░░░░██║░░██║╚█████╔╝██████╔╝╚██████╔╝╚█████╔╝███████╗██║░░██║   
╚═╝░░░░░╚═╝░░╚═╝░╚════╝░╚═════╝░░╚═════╝░░╚════╝░╚══════╝╚═╝░░╚═╝   
    
██████╗░██╗░░░██╗░██████╗████████╗  
██╔══██╗██║░░░██║██╔════╝╚══██╔══╝  
██████╔╝██║░░░██║╚█████╗░░░░██║░░░  
██╔══██╗██║░░░██║░╚═══██╗░░░██║░░░  
██║░░██║╚██████╔╝██████╔╝░░░██║░░░  
╚═╝░░╚═╝░╚═════╝░╚═════╝░░░░╚═╝░░░  
```

## 1. Start Redis local instance in Docker container:
```
    sudo docker run --name some-redis -p 6379:6379 -d redis
```

## 2. Start Redis Insights in Docker container:
```
    sudo docker run --name insights-server -v redisinsight:/db --network host -p 8001:8001 -d redislabs/redisinsight:latest"
```

## 3. Create .env file with the following environment variables:
```
    REDIS_HOST
    STREAM_NAME
```


## 4. Import .env variables using the following command (Ubuntu):
```
    export $(grep -v '^#' .env | xargs)
```

## 5. Use the following commands to check, build and run the proyect:
```
    cargo check
    cargo build
    cargo build --release (Building for release!)
    cargo run
```

To create a cool text for README.md files, visit: https://fsymbols.com/generators/carty/

Visit -> https://doc.rust-lang.org/book/