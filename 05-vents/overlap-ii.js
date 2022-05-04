import { readLines } from "https://deno.land/std/io/bufio.ts";

const positions = new Map();

for await (const line of readLines(Deno.stdin)) {
  const [start, end] = line.split(" -> ");
  const [x1, y1] = start.split(",").map((n) => +n);
  const [x2, y2] = end.split(",").map((n) => +n);

  const xi = ((x2 - x1) / Math.abs(x2 - x1)) || 0;
  const yi = ((y2 - y1) / Math.abs(y2 - y1)) || 0;

  let xn = x1;
  let yn = y1;

  while (
    xn >= Math.min(x1, x2) &&
    xn <= Math.max(x1, x2) &&
    yn >= Math.min(y1, y2) &&
    yn <= Math.max(y1, y2)
  ) {
    const pn = `${xn},${yn}`;
    positions.set(pn, 1 + (positions.get(pn) || 0));
    xn += xi;
    yn += yi;
  }
}

const numOverlaps = () => {
  return [...positions].filter(([_, count]) => count > 1).length;
};

console.log(numOverlaps());
