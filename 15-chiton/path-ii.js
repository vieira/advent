import { readLines } from "https://deno.land/std/io/bufio.ts";

class PriorityMap {
  #map = new Map();
  #queue = [];
  #key;

  constructor({ key = "priority" } = {}) {
    this.#key = key;
  }

  has(k) {
    return this.#map.has(k);
  }

  set(k, v) {
    return this.#map.set(k, v);
  }

  get(k) {
    return this.#map.get(k);
  }

  delete(k) {
    return this.#map.delete(k);
  }

  pop() {
    while (this.#queue.length) {
      const [el] = this.#queue.shift();
      if (this.#map.has(el)) {
        return [el, this.#map.get(el)];
      }
    }
  }

  push(k, v) {
    this.#map.set(k, v);

    const priority = v[this.#key];

    if (!this.#queue.length) {
      this.#queue.push([k, priority]);
    }

    let low = 0;
    let high = this.#queue.length;

    while (true) {
      const mid = Math.floor(low + ((high - low) / 2));
      const [, mprio] = this.#queue[mid];

      if (high - low <= 1) {
        for (let i = low; i < high; ++i) {
          const [, iprio] = this.#queue[i];
          if (priority < iprio) {
            this.#queue.splice(i, 0, [k, priority]);
            return;
          }
        }
        this.#queue.splice(high, 0, [k, priority]);
        return;
      }

      if (priority === mprio) {
        this.#queue.splice(mid, 0, [k, priority]);
        return;
      }

      if (priority < mprio) {
        high = mid;
        continue;
      }

      if (priority > mprio) {
        low = mid;
        continue;
      }
    }
  }
}

let cavern = [];
const unvisited = new PriorityMap({ key: "tentativeCost" });

const size = function () {
  const h = cavern.length;
  const w = cavern[0].length;
  return [w, h];
};

const extend = function (map, n = 5) {
  const [w, h] = size(map);
  for (let i = 1; i < n; ++i) {
    for (let j = 0; j < h; ++j) {
      for (let l = 0; l < w; ++l) {
        const v = map[j][l] + i;
        map[j].push(v > 9 ? (v % 10) + 1 : v);
      }
    }
  }
  for (let i = 1; i < n; ++i) {
    for (let j = 0; j < h; ++j) {
      map.push(map[j].map((v) => (v + i) > 9 ? ((v + i) % 10) + 1 : v + i));
    }
  }
  return map;
};

const adjacents = function (x, y) {
  const [w, h] = size(cavern);
  const up = y - 1 >= 0 && [x, y - 1];
  const down = y + 1 < h && [x, y + 1];
  const left = x - 1 >= 0 && [x - 1, y];
  const right = x + 1 < w && [x + 1, y];
  return [up, down, left, right].filter(Boolean);
};

const isGoal = function (x, y) {
  const [w, h] = size(cavern);
  return x === w - 1 && y === h - 1;
};

const findBestPath = function () {
  while (true) {
    const current = unvisited.pop();
    const [posKey, prt] = current;
    const position = posKey.split(",").map((n) => +n);

    adjacents(...position).forEach((nbr) => {
      const np = nbr.join(",");
      if (!unvisited.has(np)) return;
      const nc = unvisited.get(np);
      const oldCost = nc.tentativeCost;
      const newCost = prt.tentativeCost + nc.cost;
      const tentativeCost = Math.min(oldCost, newCost);
      unvisited.push(np, { ...nc, tentativeCost });
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

cavern = extend(cavern);
cavern.forEach((row, y) => {
  row.forEach((cost, x) => {
    unvisited.set([x, y].join(","), { cost, tentativeCost: Infinity });
  });
});
unvisited.push("0,0", { cost: 0, tentativeCost: 0 });

const [, { tentativeCost: cost }] = findBestPath();
console.log(cost);
