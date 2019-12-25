# tonic-limit-bug

Exits successfully:

    cargo build --release && cargo run --release --bin test 400
    
Usually fails with "Client: connection error: broken pipe":

    cargo build --release && cargo run --release --bin test 500 
