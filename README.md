## Trading back test API

### Requirement environments
docker

### Directory structure
```
.
├── docker : docker setting files
├── requests :executable http client file
└── root : application resources
        ├── envs : environment variable files
        ├── migration : database sql files
        └── src : application codes
            ├── config : config codes
            ├── controllers : direct endpoint functions
            ├── domain : domain value_objects and service, repository interfaces
            ├── infrastructure : repository concrete layer and database models
            ├── main.rs: application root file
            └── schema.rs: database schema file.
```

### How to use
Make environment directory
```
mkdir root/src/envs
```

Make environment variables file
```
touch root/src/envs/.env
```
```
SERVER_ADDR=0.0.0.0:8000
DATABASE_URL=postgres://postgres:postgres@stock-backtester-postgres:5432/stock_backtester
```

Build and run docker compose containers
```
docker compose up -d
```

Send request to use requests/dev.http request commands.

### Extensional commands

Format rust codes
```
cargo-fmt --all
```

Update database schema file
Cation: If you add something command manually, this command might overwrite these.
```
diesel migration run
```

### Other
If you have any request, please write an issues(https://github.com/explorrrr/stock-backtester/issues).
