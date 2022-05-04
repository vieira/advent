import { readLines } from "https://deno.land/std/io/bufio.ts";

let template;
const rules = new Map();

const polymeraze = function (polymer) {
  let p = "";
  for (let i = 0; i < polymer.length; i += 1) {
    const pair = polymer.slice(i, i + 2);

    if (!rules.has(pair)) {
      p += pair;
      continue;
    }

    const [left] = pair.split("");
    p += left + rules.get(pair);
  }
  return p;
};

const steps = function (polymer, steps = 10) {
  let p = polymer;
  for (let i = 0; i < steps; i += 1) {
    p = polymeraze(p);
  }
  return p;
};

const countElements = function (polymer) {
  const elements = new Map();

  for (const el of polymer) {
    elements.set(el, elements.has(el) ? 1 + elements.get(el) : 1);
  }

  return elements;
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

const counts = countElements(steps(template, 10));
console.log(max(counts) - min(counts));
