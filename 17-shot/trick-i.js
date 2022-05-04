import { readLines } from "https://deno.land/std/io/bufio.ts";

let target;

for await (const line of readLines(Deno.stdin)) {
  const [, coordinates] = line.split(":").map((s) => s.trim());
  target = coordinates.split(",").map((ax) => {
    const [_, interval] = ax.split("=");
    const [start, end] = interval.split("..").map((n) => +n);
    return [start, end];
  });
}

const position = function (v, t) {
  return -(1 / 2) * t * ((t - 1) - 2 * v);
};

const vx = function (x1, x2) {
  let t = 0;
  let v = 0;
  let pos = 0;

  while (true) {
    t = 0;
    pos = 0;
    v += 1;
    while (true) {
      t += 1;
      const nextPosition = position(v, t);
      if (t === 1 && nextPosition > x2) return vs;
      if (nextPosition <= pos) break;
      pos = nextPosition;
      if (pos > x2) break;
      if (pos >= x1) return [v, t];
    }
  }
};

const vy = function (y1, y2, [, steps]) {
  let t = steps - 1;
  let v = 0;
  const vs = [];

  while (true) {
    v = 0;
    t += 1;
    while (true) {
      v += 1;
      const finalPosition = position(v, t);
      if (finalPosition >= y1 && finalPosition <= y2) {
        vs.push([v, t]);
      }
      if (v > Math.abs(y1)) break;
    }
    if (t > 2 * v) break;
  }
  return vs.reduce(([max, steps], [v, t]) => {
    if (v > max) return [v, t];
    return [max, steps];
  }, [0]);
};

const maxAlt = function (v) {
  let currAlt = 0;
  let t = 0;

  while (true) {
    t += 1;
    const newAlt = position(v, t);
    if (newAlt <= currAlt) {
      return currAlt;
    }
    currAlt = newAlt;
  }
};

const [tx, ty] = target;
console.log(maxAlt(...vy(...ty, vx(...tx))));
