# SQLite Reader

This is a simple command line utility for interacting with SQLite database files. Install `cargo` on your system, then build it with the `cargo build -r` command for a release version. The binary will be located in `target/release/` after being built. 

## Usage

To show the help message run
```cmd
$ reader -h

Usage: sqlite-reader [options]
Options:
  -f, --db <file>       SQLite database file
  -t, --table <name>    Table name to read
  -a, --action <name>   Action to perform (list)
  -c, --columns <cols>  Comma separated list of columns to read
  -h, --help            Show this help message
```

### Usage Examples

To list all the tables in database.db:

```cmd
$ reader -f database.db -l
Tables in database.db
----------------------------------
users
products
invoices
...
```

To list all the columns in the `users` table

```cmd
$ reader -f database.db -t users -l
Columns in users
----------------------------------
_id
name
address
email
...
```
---

To print the contents of the `users` table
 
```cmd
$ reader -f database.db -t users
_id,name,address,email,age,gender,ssn,mother_maiden_name
1,Fred Smith,123 Road Street,fsmith@email.com,63,male,123-45-6789,Realname
2,Francine,152 Street Pl.,FRANCINE@yahoo.com,25,,987-65-4321,McMannus
...
```

To print the contents of the `users` table, only including specific columns, use the `-c` tag with a comma separated list of column names:
```cmd
$ reader -f database.db -t users -c _id,email
_id,email
1,fsmith@email.com
2,FRANCINE@yahoo.com
...
```
---

While the output is CSV formatted, *only simple escaping of `,` characters happens in the database contents*, so if your table cells contain comma values, the output may not work correctly

To direct reader output to a CSV, pipe the output into a file:
```cmd
$ reader -f database.db -t users > output.csv
```