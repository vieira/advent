import { readLines } from "https://deno.land/std/io/bufio.ts";

let input;

const hex2bin = function (hex) {
  return hex.split("").flatMap((n) => (
    Number.parseInt(n, 16)
      .toString(2)
      .padStart(4, "0")
      .split("")
      .map((n) => +n)
  ));
};

for await (const line of readLines(Deno.stdin)) {
  input = hex2bin(line);
}

const literal = function (bits) {
  let value = "";

  while (bits.length) {
    const [hasMoreWords] = bits.splice(0, 1);

    if (!hasMoreWords) {
      value += bits.splice(0, 4).join("");
      return Number.parseInt(value, 2);
    }

    value += bits.splice(0, 4).join("");
  }
};

const sum = function (acc = 0, v) {
  return acc + v;
};

const product = function (acc = 1, v) {
  return acc * v;
};

const minimum = function (acc = Infinity, v) {
  return Math.min(acc, v);
};

const maximum = function (acc = 0, v) {
  return Math.max(acc, v);
};

const greater = function (_acc, _v, _idx, [x1, x2]) {
  return x1 > x2 ? 1 : 0;
};

const less = function (_acc, _v, _idx, [x1, x2]) {
  return x1 < x2 ? 1 : 0;
};

const equal = function (_acc, _v, _idx, [x1, x2]) {
  return x1 === x2 ? 1 : 0;
};

const operator = function (fn) {
  return (bits) => {
    const [lengthType] = bits.splice(0, 1);
    let length = lengthType === 0
      ? Number.parseInt(bits.splice(0, 15).join(""), 2)
      : Number.parseInt(bits.splice(0, 11).join(""), 2);
    const subpackets = [];

    while (length > 0) {
      const l1 = bits.length;
      const pkt = packet(bits);
      if (pkt === null) break;
      const l2 = bits.length;
      subpackets.push(pkt);
      length -= lengthType === 0 ? l1 - l2 : 1;
    }
    return subpackets.reduce(fn);
  };
};

const packet = function (bits) {
  if (bits.every((bit) => bit === 0)) {
    return null;
  }

  const _version = Number.parseInt(bits.splice(0, 3).join(""), 2);
  const type = Number.parseInt(bits.splice(0, 3).join(""), 2);

  switch (type) {
    case 0:
      return operator(sum)(bits);
    case 1:
      return operator(product)(bits);
    case 2:
      return operator(minimum)(bits);
    case 3:
      return operator(maximum)(bits);
    case 4:
      return literal(bits);
    case 5:
      return operator(greater)(bits);
    case 6:
      return operator(less)(bits);
    case 7:
      return operator(equal)(bits);
    default:
      return null;
  }
};

console.log(packet(input));
