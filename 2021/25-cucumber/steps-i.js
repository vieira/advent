import { readLines } from "https://deno.land/std/io/bufio.ts";

const map = new Map();
let size = {};

const tuple = function (x, y) {
  return `${x},${y}`;
};

const step = function () {
  let hasMoved = false;

  for (let y = 0; y < size.y; y += 1) {
    const isLeftFree = !map.has(tuple(0, y));

    for (let x = 0; x < size.x; x += 1) {
      const xNext = (x + 1) % size.x;

      if (xNext === 0 && !isLeftFree) continue;
      if (map.get(tuple(x, y)) === ">" && !map.has(tuple(xNext, y))) {
        map.delete(tuple(x, y));
        map.set(tuple(xNext, y), ">");
        hasMoved = true;
        x += 1;
      }
    }
  }

  for (let x = 0; x < size.x; x += 1) {
    const isTopFree = !map.has(tuple(x, 0));
    for (let y = 0; y < size.y; y += 1) {
      const yNext = (y + 1) % size.y;

      if (yNext === 0 && !isTopFree) continue;
      if (map.get(tuple(x, y)) === "v" && !map.has(tuple(x, yNext))) {
        map.delete(tuple(x, y));
        map.set(tuple(x, yNext), "v");
        hasMoved = true;
        y += 1;
      }
    }
  }

  return hasMoved;
};

const steps = function () {
  let i = 1;
  while (step()) i += 1;
  return i;
};

let i = 0;
let j = 0;
for await (const line of readLines(Deno.stdin)) {
  for (j = 0; j < line.length; j += 1) {
    switch (line[j]) {
      case ">":
        map.set(tuple(j, i), ">");
        break;
      case "v":
        map.set(tuple(j, i), "v");
        break;
      default:
    }
  }
  i += 1;
}
size = { x: j, y: i };
console.log(steps());
