import { readLines } from "https://deno.land/std/io/bufio.ts";

const folds = [];
const initDots = new Map();

const addDot = function (dots, x, y) {
  if (dots.has(x)) {
    dots.get(x).add(y);
  } else {
    dots.set(x, new Set([y]));
  }
};

const addFold = function (axis, v) {
  folds.push([axis === "x" ? (x) => x : invert, v]);
};

const invert = function (dots) {
  const idots = new Map();
  dots.forEach((col, x) => {
    col.forEach((y) => {
      addDot(idots, y, x);
    });
  });
  return idots;
};

const makeFold = function (dots, v) {
  const fdots = new Map();
  const size = Math.max(...dots.keys());
  for (let i = 0; i <= size; ++i) {
    if (i === v) continue;
    const tgt = i > v ? v - (i - v) : i;
    dots.get(i)?.forEach((y) => {
      addDot(fdots, tgt, y);
    });
  }
  return fdots;
};

const countDots = function (map) {
  return Array.from(map).reduce((acc, [_, col]) => acc + col.size, 0);
};

for await (const line of readLines(Deno.stdin)) {
  if (!line) continue;

  if (line.startsWith("fold")) {
    const [, , coord] = line.split(" ");
    const [axis, v] = coord.split("=");
    addFold(axis, +v);
  } else {
    const [x, y] = line.split(",");
    addDot(initDots, +x, +y);
  }
}

const [fold] = folds;
const [rotate, v] = fold;

console.log(countDots(makeFold(rotate(initDots), v)));
