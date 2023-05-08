import { readLines } from "https://deno.land/std/io/bufio.ts";

const cavern = [];
const unvisited = new Map();

const adjacents = function (x, y) {
  const h = cavern.length;
  const w = cavern[0].length;
  const up = y - 1 >= 0 && [x, y - 1];
  const down = y + 1 < h && [x, y + 1];
  const left = x - 1 >= 0 && [x - 1, y];
  const right = x + 1 < w && [x + 1, y];
  return [up, down, left, right].filter(Boolean);
};

const isGoal = function (x, y) {
  const h = cavern.length;
  const w = cavern[0].length;
  return x === w - 1 && y === h - 1;
};

const findMinCost = function (nodes) {
  const [first] = nodes.entries();
  return Array.from(nodes).reduce(([minPos, minNode], [pos, node]) => {
    return node.tentativeCost < minNode.tentativeCost
      ? [pos, node]
      : [minPos, minNode];
  }, first);
};

const findBestPath = function () {
  while (true) {
    const current = findMinCost(unvisited);
    const [posKey, prt] = current;
    const position = posKey.split(",").map((n) => +n);

    adjacents(...position).forEach((nbr) => {
      const np = nbr.join(",");
      if (!unvisited.has(np)) return;
      const nc = unvisited.get(np);
      const oldCost = nc.tentativeCost;
      const newCost = prt.tentativeCost + nc.cost;
      unvisited.set(np, { ...nc, tentativeCost: Math.min(oldCost, newCost) });
    });

    unvisited.delete(posKey);

    if (isGoal(...position)) {
      return current;
    }
  }
};

for await (const line of readLines(Deno.stdin)) {
  cavern.push(line.split("").map((n) => +n));
}

cavern.forEach((row, y) => {
  row.forEach((cost, x) => {
    unvisited.set([x, y].join(","), { cost, tentativeCost: Infinity });
  });
});

unvisited.set("0,0", { cost: 0, tentativeCost: 0 });

const [, { tentativeCost: cost }] = findBestPath();
console.log(cost);
