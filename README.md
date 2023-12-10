### Simple Grep CLI App Build with Rust

A simple cli app build using rust, to grep a user input pattern in a user input file

```sh
<path/to/bin/>simplegrep foostring path_to_text_file
```

## Improvments and Next goals
- Use `BufReader` to improve read speeds for the input document
- Use a cleaner output format to show the results
- It will be interesting to understand statistics of the pattern
- Can we do more than 1 pattern as an input
- Can we do a directory of Files as an input (as well keeping a file as an input)
- Add some better CLI documnets such as `--help` and `--version`
