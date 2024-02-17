# gift_circle

## Expanation

The gift_circle software reads a CSV file of gift circle participants and generates gift-recipient assignments.

You can use it to generate a simple list of recipients where no groups are involved, or you can use groups where each particiant will be assigned a person to get a gift for that is not within their own group.

The groups are assumed to be family groups where it would be difficult to purchase a gift for someone in that group and still keep it a secret given the proximity and exposure of the people in that group. You must invoke the -u/--use_groups flag to cause the program to use groups. If the flag is not invoked, it assumes no groups and randomly picks a recipent based upon everyone not previously picked.

The input file must be in UTF-8 or UTF-8-compatible encoding (e.g., ASCII is UTF-8 compatible, but Latin-1 is not).

### Not Using Groups

- Set up a plain CSV file with a header row and data rows, ensuring there is data for each column. You must use these exact column names, but you can choose to use either name only or name and email-address. All names must be unique and email-address is simply passed through to the output CSV. See `example-participants-without-groups.csv` for the format.

  -- name

  -- name,email_address

- Invoke the program with the input file location, using the short or long option format. See the available options using -h/--help).

#### Invoking Without Groups

```shell
./gift_circle -i=path/to/participants.csv
./gift_circle --input=./participants.csv
```

```shell
#INFO: Found valid gift circle NOT USING groups in 1 attempts
name,email_address,assigned_person_name
Jane Hill,,Jack Brown
Jack Brown,,Joe Hill
Joe Hill,,Daisy Jones
Daisy Jones,,Bill Jones
Bill Jones,,Beverly Jones
Beverly Jones,,Kenya Hill
Kenya Hill,,Jessica Brown
Jessica Brown,,Billy Jones
Billy Jones,,Jane Hill
```

The output will include a column for email_address whether or not you include it. This is simply to allow you to include it or not in the input file.

You can redirect the output to a file using shell redirection. You might want to choose this since some information is written to stderr during processing and redirecting stdout to a file will exclude that processing info from your final output.

```shell
./gift_circle -i=path/to/participants.csv > gift-assignments.csv
```

### Using Groups

Given that it is impossible to build a gift circle for certain combinations of groups, this software will make an initial determination of whether it's possible to proceed based upon whether the count of folks in the largest group, times two, is less than or equal to the total number of participants provided.

The format of the CSV must be as below, with this exact header row. You can leave out the email_address header and column info (along with its delimiting comma) if desired.

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

#### Invoking With Groups

```shell
./gift_circle -u -i=path/to/participants.csv
./gift_circle --use-groups --input=./participants.csv
./gift_circle -u -i=./participants.csv > ./gift-assignments.csv
```

The software will read the participants file, create a gift circle using those participants, and output a new CSV to stdout that includes a new column (assigned_person_name) showing the person they are assigned to get a gift for.

Here is an example output. Note the order of the group numbers.

```shell
#INFO: Found valid gift circle USING groups in 2 attempts
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
