# Integration Tests

The tests use "NEAR Workspaces" testing framework.

Official doc: [https://docs.near.org/develop/testing/integration-test](https://docs.near.org/develop/testing/integration-test)  
GitHub: [https://docs.near.org/develop/testing/integration-test](https://github.com/near/workspaces-rs)

## Setup

```sh
cp build-contracts.example.sh build-contracts.sh
chmod +x build-contracts.sh
chmod +x run.sh
```

## Run the tests

```bash
./run.sh
```

or

```bash
cargo run --example integration-tests
```
