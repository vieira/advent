import { readLines } from "https://deno.land/std/io/bufio.ts";

let positions;

for await (const line of readLines(Deno.stdin)) {
  positions = line.split(",").map((t) => +t).sort((x, y) => x - y);
}

const median = function () {
  const position = Math.round(positions.length / 2);
  return positions[position];
};

const tgt = median();
const fuel = positions.reduce((acc, pos) => Math.abs(pos - tgt) + acc, 0);
console.log(fuel);
