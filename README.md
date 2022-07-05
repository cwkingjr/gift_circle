# gift_circle

## Expanation

The gift_circle software reads a CSV file of gift circle participants and generates gift assignments where each particiant will
be assigned a person to get a gift for that is not within their own group.

The groups are expected to be family groups where it would be difficult to purchase a gift for someone in that group and keep still keep it a secret given the proximity and exposure of the people in that group.

Given that it is impossible to build a gift circle for certain combinations of groups, this software will make an initial determination of whether it's possible to proceed based upon whether the count of folks in the largest group, times two, is less than or equal to the total number of participants provided.

The format of the CSV must be as below, with this exact header row, with file in UTF-8 or UTF-8 compatible encoding (e.g., ASCII is UTF-8 compatible, but Latin-1 is not).

```shell
name,email_address,group_number
Father,father@example.com,1
Mother,mother@example.com,1
Son 1,son1@example.com,2
Daughter 2,duaghter2@example.com,2
Daughter 1,daughter1@example.com,3
Son 2,son2@example.com,3
```

## Use

```shell
./gift_circle path/to/participants.csv
./gift_circle ./participants.csv
./gift_circle ./participants.csv > ./gift-giving-order.csv
```

The software will read the participants file, create a gift circle using those participants, and output a new CSV to stdout that places participants in the order of gift giving. E.g., the first participant in the output will purchase a gift for the second participant, second will purchase for third, ..., last participant will purchase a gift for the first participant.

Here is an example output. Note the order of the group numbers.

```shell
name,email_address,group_number
Daughter 1,daughter1@example.com,3
Father,father@example.com,1
Daughter 2,duaghter2@example.com,2
Son 2,son2@example.com,3
Mother,mother@example.com,1
Son 1,son1@example.com,2
```

## Code

This code was written and compiled on a MacBook, so the binary should work on an Intel-based Mac. If that is not your environment, you can attempt to compile it yourself using Rust.

Install Rust, clone the repo into a local directory, cd into that directory, and run ```cargo build```. Then invoke the debug binary as listed above, but consuming the sample participants file. E.g., ```./target/debug/gift_circle ./src/example-participants.csv```. Then, create your own participants file and execute the binary against your file.
