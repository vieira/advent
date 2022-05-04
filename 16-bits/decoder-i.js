import { readLines } from "https://deno.land/std/io/bufio.ts";

let input;
let v = 0;

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

const operator = function (bits) {
  const [lengthType] = bits.splice(0, 1);
  const _length = lengthType === 0
    ? Number.parseInt(bits.splice(0, 15).join(""), 2)
    : Number.parseInt(bits.splice(0, 11).join(""), 2);
  const subpackets = [];

  while (bits.length) {
    const pkt = packet(bits);
    if (pkt === null) break;
    subpackets.push(pkt);
  }

  return subpackets;
};

const packet = function (bits) {
  if (bits.every((bit) => bit === 0)) {
    return null;
  }

  const version = Number.parseInt(bits.splice(0, 3).join(""), 2);
  const type = Number.parseInt(bits.splice(0, 3).join(""), 2);
  v += version;

  switch (type) {
    case 4:
      return literal(bits);
    default:
      return operator(bits);
  }
};

packet(input);
console.log(v);
