# tonic-limit-bug

Exits successfully:

    cargo build --release && cargo run --release --bin test 400
    
Usually fails with "Client: connection error: broken pipe":

    cargo build --release && cargo run --release --bin test 500 

Also exits successfully:

    cargo build --release && cargo run --release --bin test -- --multiprocess 1000
