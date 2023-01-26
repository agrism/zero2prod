
### Set up PG
```bash
 ./scripts/init_db.sh  
 # or
 SKIP_DOCKER=true ./scripts/init_db.sh 
```

### Commands

```bash
cargo test
```

```bash
cargo tarpaulin --ignore-tests
```

```bash
cargo check
```

```bash
cargo clippy
```

```bash
cargo audit
```

```bash
cargo fmt
```

```bash
cargo run
```


### routes
```bash
curl -v http://127.0.0.1:8000/health_check
```


### CI
```bash
docker build --tag zero2prod --file Dockerfile .
```