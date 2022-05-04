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

const has = function (list, [x, y]) {
  return list.find(([lx, ly]) => lx === x && ly === y);
};

const lowPoints = function () {
  return heatmap.reduce((acc, row, y) => {
    row.forEach((_, x) => {
      if (isLowest(x, y)) acc.push([x, y]);
    });
    return acc;
  }, []);
};

const findBasin = function (x, y) {
  const basin = [[x, y]];
  const successors = [[x, y]];
  while (successors.length) {
    const [bx, by] = successors.pop();
    adjacents(bx, by).forEach((adj) => {
      if (height(bx, by) > height(...adj) || height(...adj) >= 9) return;
      if (has(basin, adj)) return;

      basin.push(adj);
      successors.push(adj);
    });
  }
  return basin;
};

console.log(
  lowPoints()
    .map((pos) => findBasin(...pos))
    .map((basin) => basin.length)
    .sort((x, y) => y - x)
    .slice(0, 3)
    .reduce((acc, v) => acc * v, 1),
);
