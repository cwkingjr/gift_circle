# gift_circle

## Expanation

The gift_circle software reads a CSV file of gift circle participants and generates gift assignments where each particiant will be assigned a person to get a gift for that is not within their own group.

The groups are assumed to be family groups where it would be difficult to purchase a gift for someone in that group and still keep it a secret given the proximity and exposure of the people in that group.

Given that it is impossible to build a gift circle for certain combinations of groups, this software will make an initial determination of whether it's possible to proceed based upon whether the count of folks in the largest group, times two, is less than or equal to the total number of participants provided.

The format of the CSV must be as below, with this exact header row, with file in UTF-8 or UTF-8 compatible encoding (e.g., ASCII is UTF-8 compatible, but Latin-1 is not).

```shell
name,email_address,group_number
Joe Hill,joe.hill@example.com,1
Jane Hill,jane.hill@example.com,1
Kenya Hill,kenya.hill@example.com,1
Jack Brown,jack.brown@example.com,2
Jessica Brown,jessica.brown@example.com,2
Bill Jones,bill.jones@example.com,3
Beverly Jones,bev.jones@example.com,3
Billy Jones,billy.jones@example.com,3
Daisy Jones,daisy.jones@example.com,3
```

## Use

```shell
./gift_circle --help
./gift_circle -i=path/to/participants.csv
./gift_circle --input=./participants.csv
./gift_circle -i=./participants.csv > ./gift-assignments.csv
```

The software will read the participants file, create a gift circle using those participants, and output a new CSV to stdout that includes a new column (assigned_person_name) showing the person they are assigned to get a gift for.

Here is an example output. Note the order of the group numbers.

```shell
#INFO: Found valid circle in 3 attempts
name,email_address,group_number,assigned_person_name
Jack Brown,jack.brown@example.com,2,Joe Hill
Joe Hill,joe.hill@example.com,1,Beverly Jones
Beverly Jones,bev.jones@example.com,3,Jane Hill
Jane Hill,jane.hill@example.com,1,Bill Jones
Bill Jones,bill.jones@example.com,3,Jessica Brown
Jessica Brown,jessica.brown@example.com,2,Billy Jones
Billy Jones,billy.jones@example.com,3,Kenya Hill
Kenya Hill,kenya.hill@example.com,1,Daisy Jones
Daisy Jones,daisy.jones@example.com,3,Jack Brown
```

## Code

This code was written and compiled on an Intel-based MacBook Pro, so the release binary should work on any Intel-based Mac. If you have an Intel MacBook, you should be able to download the gift_cirle binary that is attached to the GitHub release, modify the permissions to make it executable (chmod +x gift_circle), and invoke it against your participants file as shown above.

If you have a different machine, you can install Rust for your machine, download the repo (git clone) or source code to your machine, and compile it from the repo folder: "cargo build --release". You can also run these:

```sh
cargo test
cargo run -- --help
cargo run -- --i=./src/example-participants.csv
cargo build
./target/debug/gift_circle --i=./src/example-participants.csv
```
