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

const print = function (dots) {
  const idots = invert(dots);
  const w = Math.max(...idots.keys());
  for (let i = 0; i <= w; i += 1) {
    const row = idots.get(i) ?? new Set();
    const h = Math.max(...row);
    let line = "";
    for (let j = 0; j <= h; j += 1) {
      line += row.has(j) ? "#" : " ";
    }
    console.log(line);
  }
};

const makeAllFolds = function () {
  return folds.reduce((acc, fold) => {
    const [rotate, v] = fold;
    return rotate(makeFold(rotate(acc), v));
  }, initDots);
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

print(makeAllFolds());
