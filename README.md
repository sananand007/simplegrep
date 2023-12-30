### Simple Grep CLI App Build with Rust

A simple cli app build using rust, to grep a user input pattern in a user input file

Functionalities 
- gives out the word count
- checks presence of a user input pattern

```sh
<path/to/bin/>simplegrep foostring path_to_text_file

simplegrep % cargo run -- --help
   Compiling simplegrep v0.1.0 (/Users/wano/Public/code/buildWithRust/simplegrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/simplegrep --help`

Search and logically process patterns on the Command line

Usage: simplegrep [OPTIONS]

Options:
  -r, --pattern <PATTERN>  The pattern to look for [default: ]
  -p, --path <PATH>        The path of the file to read [default: ]
  -h, --help               Print help
  -V, --version            Print version

```

## Current Output 
```
simplegrep % cargo run -- --pattern fox --path ~/Public/testArtifacts/testSimplegrep.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/simplegrep --pattern fox --path /Users/wano/Public/testArtifacts/testSimplegrep.txt`
pattern "fox" in "/Users/wano/Public/testArtifacts/testSimplegrep.txt" true
######################################
jumped:2
angry:1
were:1
fox:11
jumping:1
very:1
he:1
struck:1
to:2
A:1
on:1
now:1
then:1
was:5
and:2
forest:1
dog:8
over:1
a:1
fat:1
the:8
knew:2
house:1
not:1
returned:2
at:3
nice:2
ran:1
limped:1
when:1
smiling:1
its:1
good:2
landed:1
The:11
back:3
######################################

```

## Improvments

- Sort the output and print the top 10% 
- add arg for each functionality
- Can we do more than 1 pattern as an input
- Can we do a directory of Files as an input (as well keeping a file as an input)
- Add some better CLI documnets such as `--help` and `--version`
- Get a structured and formatted output, based on kind of output needed
- Take a input as a Large document and output a structured format
- Take a URI as an input and process that and output some logical data
- Introduce Logging
- Introduce tests 
