# gift_circle

## Expanation

The gift_circle software reads a CSV file of gift circle participants and generates gift assignments where each particiant will
be assigned a person to get a gift for that is not within their own group.

The groups are expected to be family groups where it would be difficult to purchase a gift for someone in that group and still keep it a secret given the proximity and exposure of the people in that group.

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
./gift_circle path/to/participants.csv
./gift_circle ./participants.csv
./gift_circle ./participants.csv > ./gift-giving-order.csv
```

The software will read the participants file, create a gift circle using those participants, and output a new CSV to stdout that includes a new column showing the person they are assigned to get a gift for.

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

This code was written and compiled on a MacBook, so the binary should work on an Intel-based Mac. If that is not your environment, you can attempt to compile it yourself using Rust.

Install Rust, clone the repo into a local directory, cd into that directory, and run ```cargo build```. Then invoke the debug binary as listed above, but consuming the sample participants file. E.g., ```./target/debug/gift_circle ./src/example-participants.csv```. Then, create your own participants file and execute the binary against your file.
