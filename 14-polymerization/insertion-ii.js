import { readLines } from "https://deno.land/std/io/bufio.ts";

let template;
const rules = new Map();

const pairs = function (polymer) {
  const pairsCount = new Map();

  for (let i = 0; i < polymer.length - 1; i += 1) {
    const pair = polymer.slice(i, i + 2);
    pairsCount.set(pair, pairsCount.has(pair) ? 1 + pairsCount.get(pair) : 1);
  }

  return pairsCount;
};

const polymeraze = function (pairsCount) {
  const prevPairsCount = new Map(pairsCount);
  const prevPairs = prevPairsCount.keys();

  for (const pair of prevPairs) {
    if (!rules.has(pair)) continue;

    const count = prevPairsCount.get(pair);
    const [left, right] = pair.split("");
    const p1 = left + rules.get(pair);
    const p2 = rules.get(pair) + right;
    pairsCount.set(p1, pairsCount.has(p1) ? count + pairsCount.get(p1) : count);
    pairsCount.set(p2, pairsCount.has(p2) ? count + pairsCount.get(p2) : count);

    const count2 = pairsCount.get(pair);
    if (count2 - count > 0) {
      pairsCount.set(pair, count2 - count);
    } else {
      pairsCount.delete(pair);
    }
  }

  return pairsCount;
};

const steps = function (polymer, steps = 10) {
  let p = polymer;
  for (let i = 0; i < steps; i += 1) {
    p = polymeraze(p);
  }
  return p;
};

const countElements = function (pairsCount) {
  const counts = new Map();

  Array.from(pairsCount).forEach(([pair, count]) => {
    pair.split("").forEach((el) => {
      counts.set(el, counts.has(el) ? counts.get(el) + count : count);
    });
  });

  return new Map(
    Array.from(counts).map(([el, cnt]) => [el, Math.ceil(cnt / 2)]),
  );
};

const max = function (map) {
  return Array.from(map).reduce((acc, [, v]) => Math.max(acc, v), 0);
};

const min = function (map) {
  return Array.from(map).reduce((acc, [, v]) => Math.min(acc, v), Infinity);
};

for await (const line of readLines(Deno.stdin)) {
  if (!line) continue;

  if (!template) {
    template = line;
    continue;
  }

  const [pattern, insertion] = line.split(" -> ");
  rules.set(pattern, insertion);
}

const elements = countElements(steps(pairs(template), 40));
console.log(max(elements) - min(elements));
