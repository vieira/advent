import { readLines } from "https://deno.land/std/io/bufio.ts";

class PriorityQueue {
  #queue = [];
  #priorityFn;

  constructor({ priorityFn = (x) => x } = {}) {
    this.#priorityFn = priorityFn;
  }

  length() {
    return this.#queue.length;
  }

  pop() {
    const [el] = this.#queue.shift();
    return el;
  }

  push(v) {
    const priority = this.#priorityFn(v);

    if (!this.#queue.length) {
      this.#queue.push([v, priority]);
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
            this.#queue.splice(i, 0, [v, priority]);
            return;
          }
        }
        this.#queue.splice(high, 0, [v, priority]);
        return;
      }

      if (priority === mprio) {
        this.#queue.splice(mid, 0, [v, priority]);
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

const initialMap = new Map();
const costs = {
  A: 1,
  B: 10,
  C: 100,
  D: 1000,
};

let i = 0;
for await (const line of readLines(Deno.stdin)) {
  const lines = i === 3 ? ["  #D#C#B#A#", "  #D#B#A#C#", line] : [line];

  for (const l of lines) {
    for (let j = 0; j < l.length; j += 1) {
      switch (l[j]) {
        case ".":
          initialMap.set([j, i].join(","), false);
          break;
        case "A":
        case "B":
        case "C":
        case "D":
          initialMap.set([j, i].join(","), l[j]);
          break;
        default:
      }
    }
    i += 1;
  }
}

const key = function (x, y) {
  return [x, y].join(",");
};

const canMove = function (map, x, y) {
  return map.get(key(x, y)) === false;
};

const canEntry = function (map, pod, x, y) {
  switch (pod) {
    case "A": {
      if (x !== 3) return false;
      break;
    }
    case "B": {
      if (x !== 5) return false;
      break;
    }
    case "C": {
      if (x !== 7) return false;
      break;
    }
    case "D": {
      if (x !== 9) return false;
      break;
    }
  }

  if (y > 1 && y < 5) return map.get(key(x, y + 1)) === pod;
  return true;
};

const canStop = function (map, pod, x, y, isLastMove) {
  const entrances = new Set([3, 5, 7, 9]);

  if (isLastMove && y < 2) {
    return false;
  }

  if (y === 1 && entrances.has(x)) {
    return false;
  }

  if (y > 1) {
    return canEntry(map, pod, x, y);
  }

  return true;
};

const adjacents = function (map, x, y) {
  const up = canMove(map, x, y - 1) && [x, y - 1];
  const down = canMove(map, x, y + 1) && [x, y + 1];
  const left = canMove(map, x - 1, y) && [x - 1, y];
  const right = canMove(map, x + 1, y) && [x + 1, y];
  return [up, down, left, right].filter(Boolean);
};

const move = function (map, from, to) {
  const nmap = new Map(map);
  const p1 = key(...from);
  const p2 = key(...to);
  const pod = map.get(p1);
  nmap.set(p2, pod);
  nmap.set(p1, false);
  return nmap;
};

const moves = function (m, from) {
  const ss = [[m, from, 0]];
  const ms = [];
  const visited = new Set();
  const [xf, yf] = from;
  const isLastMove = yf < 2;

  while (ss.length) {
    const [map, orig, c] = ss.shift();
    const kpos = key(...orig);
    const pod = map.get(kpos);
    const ncost = c + costs[pod];

    for (const pos of adjacents(map, ...orig)) {
      const [xt, yt] = pos;
      if (visited.has(key(...pos))) continue;
      if (xf === xt && yf === yt) continue;
      const nmap = move(map, orig, pos);
      if (canStop(map, pod, ...pos, isLastMove)) {
        if (yt > 1) return [[nmap, pos, ncost]];
        ms.push([nmap, pos, ncost]);
      }
      ss.push([nmap, pos, ncost]);
    }
    visited.add(kpos);
  }
  return ms;
};

const serialize = function (state) {
  const { map } = state;
  return Array.from(map.entries())
    .filter(([, pod]) => pod)
    .sort(([, pod1], [, pod2]) => {
      if (pod1 === pod2) return 0;
      if (pod1 > pod2) return 1;
      return -1;
    })
    .reduce((acc, [pos, pod]) => `${acc};${pos}:${pod}`, "");
};

const h1 = function (map) {
  let h = 0;

  for (const [kpos, pod] of map.entries()) {
    if (!pod) continue;
    const [x, y] = kpos.split(",").map(Number);
    const ph = y > 1 ? y : 1;
    switch (pod) {
      case "A":
        h += x !== 3 ? ph + Math.abs(3 - x) : 0;
        break;
      case "B":
        h += x !== 5 ? 10 * (ph + Math.abs(5 - x)) : 0;
        break;
      case "C":
        h += x !== 7 ? 100 * (ph + Math.abs(7 - x)) : 0;
        break;
      case "D":
        h += x !== 9 ? 1000 * (ph + Math.abs(9 - x)) : 0;
        break;
      default:
    }
  }

  return h;
};

const successors = function* (state) {
  const { map, cost } = state;

  for (const [pkey, pod] of map.entries()) {
    if (!pod) continue;
    const p = pkey.split(",").map(Number);
    const [x, y] = p;
    if (canEntry(map, pod, x, y)) continue;
    for (const [m, , c] of moves(map, p)) {
      yield ({
        map: m,
        cost: cost + c,
        h: h1(m),
      });
    }
  }
};

const isGoal = function (state) {
  const { map } = state;
  return map.get("3,2") === "A" &&
    map.get("3,3") === "A" &&
    map.get("3,4") === "A" &&
    map.get("3,5") === "A" &&
    map.get("5,2") === "B" &&
    map.get("5,3") === "B" &&
    map.get("5,4") === "B" &&
    map.get("5,5") === "B" &&
    map.get("7,2") === "C" &&
    map.get("7,3") === "C" &&
    map.get("7,4") === "C" &&
    map.get("7,5") === "C" &&
    map.get("9,2") === "D" &&
    map.get("9,3") === "D" &&
    map.get("9,4") === "D" &&
    map.get("9,5") === "D";
};

const findBest = function (state) {
  const ss = new PriorityQueue({ priorityFn: ({ cost, h }) => cost + h });
  const visited = new Set();
  ss.push(state);

  while (ss.length) {
    const current = ss.pop();
    const sc = serialize(current);
    if (visited.has(sc)) continue;
    if (isGoal(current)) return current;
    for (const s of successors(current)) {
      ss.push(s);
    }
    visited.add(sc);
  }
};

const { cost } = findBest({ map: initialMap, cost: 0, h: 0 });
console.log(cost);
