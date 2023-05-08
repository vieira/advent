# Advent of Code

Personal repository of [Advent of Code](https://adventofcode.com) solutions.

## Run solutions

Each folder contains two standalone files, ending in `-i.<ext>` for part 1,
`-ii.<ext>` for part 2 and `input.txt`.

Each program will read the input from stdin and output the solution to stdout.

To run the first day of 2021:

```sh
cd 2021/01-sonar
deno run sweep-i.js < input.txt
```

and to run the last day of 2022:

```sh
cd 2022/25-fuel
go run snafu-i.go < input.txt
```
