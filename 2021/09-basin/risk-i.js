import { readLines } from "https://deno.land/std/io/bufio.ts";

const heatmap = [];

for await (const line of readLines(Deno.stdin)) {
  heatmap.push(line.split("").map((h) => +h));
}

const height = function (x, y) {
  return heatmap[y][x];
};

const adjacents = function (x, y) {
  const h = heatmap.length;
  const w = heatmap[0].length;
  const up = y - 1 >= 0 && [x, y - 1];
  const down = y + 1 < h && [x, y + 1];
  const left = x - 1 >= 0 && [x - 1, y];
  const right = x + 1 < w && [x + 1, y];
  return [up, down, left, right].filter(Boolean);
};

const isLowest = function (x, y) {
  return adjacents(x, y).every((pos) => height(...pos) > height(x, y));
};

const totalRisk = function () {
  return heatmap.reduce((acc, row, y) => (
    acc + row.reduce((acc, h, x) => (
      isLowest(x, y) ? acc + (1 + h) : acc
    ), 0)
  ), 0);
};

console.dir(totalRisk());
