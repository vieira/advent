import { readLines } from "https://deno.land/std/io/bufio.ts";

let positions;

for await (const line of readLines(Deno.stdin)) {
  positions = line.split(",").map((t) => +t).sort((x, y) => x - y);
}

const calcFuel = function (pos, tgt) {
  const n = Math.abs(pos - tgt);
  const an = 1 + (n - 1) * 1;
  return n * (1 + an) / 2;
};

const minFuel = function () {
  const tgts = [
    ...Array(positions[positions.length - 1] - positions[0]).keys(),
  ];
  return tgts.reduce(([minFuel, bestTgt], tgt) => {
    const totalFuel = positions.reduce((acc, p) => calcFuel(p, tgt) + acc, 0);
    if (totalFuel < minFuel) return [totalFuel, tgt];
    return [minFuel, bestTgt];
  }, [Infinity, null]);
};

const [fuel] = minFuel();
console.log(fuel);
