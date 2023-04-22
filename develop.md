# Develop

This code was written and compiled on an Intel-based MacBook Pro (Ventura 13.0.1). If you have an Intel MacBook, you should be able to download the gift_cirle binary that is attached to the GitHub release, modify the permissions to make it executable (chmod +x gift_circle), and invoke it against your participants file as shown above.

Note that the code is compiled to Ubuntu Linux via a GitHub action during the pull request process, but no build Linux binary is included within the release.

If you have a different machine, you can install Rust for your machine, download the repo (git clone) or source code to your machine, and compile it from the repo folder: `cargo build --release`.

You can also run these:

```sh
cargo test
cargo build
cargo run -- --help
cargo run -- -u -i=./data/example-participants-with-groups.csv
cargo run -- -i=./data/example-participants-without-groups.csv
cargo run ./target/debug/gift_circle -u -i=./data/example-participants-with-groups.csv
```

Once the gift_circle binary is moved into your path (e.g., /usr/bin/gift_circle), of course you may invoke it like this:

```sh
gift_circle -h
gift_circle -i=./my-participants.csv
gift_circle -a -i=~./my-participants.csv
gift_circle -a -u -i=~./my-participants.csv
```
