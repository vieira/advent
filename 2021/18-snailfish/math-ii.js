import { readLines } from "https://deno.land/std/io/bufio.ts";

const numbers = [];

const value = function (node) {
  if (node?.value === 0) {
    return 0;
  }
  return node?.value ?? node.map(value);
};

const tree2flat = function (value, depth = 0) {
  if (typeof value === "number") {
    return { value, depth };
  }
  return value.flatMap((x) => tree2flat(x, depth + 1));
};

const flat2tree = function (array) {
  if (array.length === 2) {
    return value(array);
  }
  for (let i = 0; i < array.length - 1; i++) {
    const [
      { value: v1, depth: d1 },
      { value: v2, depth: d2 },
    ] = array.slice(i, i + 2);

    if (d1 === d2) {
      return flat2tree([
        ...array.slice(0, i),
        { value: [v1, v2], depth: d1 - 1 },
        ...array.slice(i + 2),
      ]);
    }
  }
  return array;
};

const explode = function (n) {
  for (let i = 0; i < n.length - 1; i += 1) {
    const d1 = n[i];
    const d2 = n[i + 1];

    if (d1.depth !== d2.depth || d1.depth <= 4) continue;

    if (i > 0) {
      n[i - 1].value += n[i].value;
    }

    if (i < n.length - 2) {
      n[i + 2].value += n[i + 1].value;
    }

    n[i + 1].value = 0;
    n[i + 1].depth -= 1;

    n.splice(i, 1);
    return true;
  }

  return false;
};

const split = function (n) {
  for (let i = 0; i < n.length; i += 1) {
    const d = n[i];

    if (d.value < 10) continue;

    n.splice(i, 1, { value: Math.floor(d.value / 2), depth: d.depth + 1 });
    n.splice(i + 1, 0, { value: Math.ceil(d.value / 2), depth: d.depth + 1 });
    return true;
  }
  return false;
};

const reduce = function (n) {
  while (explode(n) || split(n));
  return n;
};

const sum = function (ns) {
  return ns.reduce((acc, n) => flat2tree(reduce(tree2flat([acc, n]))));
};

const magnitude = function (t) {
  if (Number.isInteger(t)) return t;
  const [t1, t2] = t;
  return 3 * magnitude(t1) + 2 * magnitude(t2);
};

const largestMagnitudeSum = function (ns) {
  let max = 0;

  for (let i = 0; i < ns.length; i += 1) {
    for (let j = 0; j < ns.length; j += 1) {
      if (i === j) continue;
      max = Math.max(max, magnitude(sum([ns[i], ns[j]])));
    }
  }

  return max;
};

for await (const line of readLines(Deno.stdin)) {
  numbers.push(eval(line));
}

console.log(largestMagnitudeSum(numbers));
