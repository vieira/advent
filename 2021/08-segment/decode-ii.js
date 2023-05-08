import { readLines } from "https://deno.land/std/io/bufio.ts";

const signals = [];

for await (const line of readLines(Deno.stdin)) {
  const [input, output] = line.split(" | ");
  signals.push([input, output].map((x) => x.split(" ")));
}

const sort = function (signal) {
  return signal.split("").sort().join("");
};

const contains = function (s1, s2) {
  return s2.split("").every((c) => s1.includes(c));
};

const sum = function (numbers) {
  return numbers.reduce((acc, v) => acc + v, 0);
};

const lengths = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

const computeEncoding = function ([signal]) {
  const known = new Map();
  const candidates = new Map();
  signal.forEach((digit) => {
    lengths.forEach((length, num) => {
      if (length === digit.length) {
        const cs = candidates.get(sort(digit)) ?? new Set();
        candidates.set(sort(digit), cs.add(num));
      }
    });
  });
  candidates.forEach((nums, digit) => {
    if (nums.size === 1) {
      const [num] = nums;
      known.set(num, digit);
      candidates.delete(digit);
    }
  });
  candidates.forEach((_, digit) => {
    if (digit.length === 6) {
      if (contains(digit, known.get(4))) {
        known.set(9, digit);
      } else if (contains(digit, known.get(1))) {
        known.set(0, digit);
      } else {
        known.set(6, digit);
      }
      candidates.delete(digit);
    }
  });
  candidates.forEach((_, digit) => {
    if (digit.length === 5) {
      if (contains(digit, known.get(1))) {
        known.set(3, digit);
      } else if (contains(known.get(9), digit)) {
        known.set(5, digit);
      } else {
        known.set(2, digit);
      }
      candidates.delete(digit);
    }
  });
  return new Map(Array.from(known, (a) => a.reverse()));
};

const decodeSignal = function ([, signal], encoding) {
  return +signal.map((d) => encoding.get(sort(d))).join("");
};

console.log(sum(signals.map((s) => decodeSignal(s, computeEncoding(s)))));
