import { readLines } from "https://deno.land/std/io/bufio.ts";

let octopuses = [];
let flashCount = 0;

for await (const line of readLines(Deno.stdin)) {
  octopuses.push(line.split("").map((h) => +h));
}

const energy = function (x, y) {
  return octopuses[y][x];
};

const increaseEnergy = function (x, y) {
  if (octopuses[y][x] >= 9) {
    flashCount += 1;
    octopuses[y][x] = -1;
  } else {
    octopuses[y][x] += 1;
  }
  return octopuses[y][x];
};

const adjacents = function (x, y) {
  const h = octopuses.length;
  const w = octopuses[0].length;
  const top = y - 1 >= 0 && [x, y - 1];
  const bottom = y + 1 < h && [x, y + 1];
  const left = x - 1 >= 0 && [x - 1, y];
  const right = x + 1 < w && [x + 1, y];
  const topLeft = y - 1 >= 0 && x - 1 >= 0 && [x - 1, y - 1];
  const topRight = y - 1 >= 0 && x + 1 < w && [x + 1, y - 1];
  const bottomLeft = y + 1 < h && x - 1 >= 0 && [x - 1, y + 1];
  const bottomRight = y + 1 < h && x + 1 < w && [x + 1, y + 1];
  return [
    top,
    bottom,
    left,
    right,
    topLeft,
    topRight,
    bottomLeft,
    bottomRight,
  ].filter(Boolean);
};

const propagateFlash = function (x, y) {
  const successors = [[x, y]];
  while (successors.length) {
    const [fx, fy] = successors.pop();
    adjacents(fx, fy).forEach(([ax, ay]) => {
      if (energy(ax, ay) < 0) return;
      const hasFlashed = increaseEnergy(ax, ay) < 0;
      if (hasFlashed) successors.push([ax, ay]);
    });
  }
};

const step = function () {
  octopuses.forEach((row, y) => {
    row.forEach((octopus, x) => {
      if (octopus < 0) return;
      const hasFlashed = increaseEnergy(x, y) < 0;
      if (hasFlashed) propagateFlash(x, y);
    });
  });
  octopuses = octopuses.map((row) => row.map((o) => Math.max(o, 0)));
  return octopuses;
};

for (let i = 0; i < 100; i += 1) {
  step();
}

console.log(flashCount);
