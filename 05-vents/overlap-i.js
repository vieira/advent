import { readLines } from "https://deno.land/std/io/bufio.ts";

const positions = new Map();

for await (const line of readLines(Deno.stdin)) {
  const [start, end] = line.split(" -> ");
  const [x1, y1] = start.split(",").map((n) => +n);
  const [x2, y2] = end.split(",").map((n) => +n);

  if (x1 === x2) {
    for (let yn = Math.min(y1, y2); yn <= Math.max(y1, y2); ++yn) {
      const pos = `${x1},${yn}`;
      positions.set(pos, 1 + (positions.get(pos) ?? 0));
    }
  }

  if (y1 === y2) {
    for (let xn = Math.min(x1, x2); xn <= Math.max(x1, x2); ++xn) {
      const pos = `${xn},${y1}`;
      positions.set(pos, 1 + (positions.get(pos) ?? 0));
    }
  }
}

const numOverlaps = () => {
  return [...positions].filter(([_, count]) => count > 1).length;
};

console.log(numOverlaps());
