# Songquiz

Songquiz for the UHH Informatics Songquiz AG

## How to use

In the root directory, run

```
cargo build -p client --target wasm32-unknown-unknown
cargo build -p songquiz
```

Now run the produced executable with the environment variables `RSPOTIFY_CLIENT_ID`, `RSPOTIFY_CLIENT_SECRET`, `RSPOTIFY_CLIENT_REDIRECT` set