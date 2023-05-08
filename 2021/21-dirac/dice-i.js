import { readLines } from "https://deno.land/std/io/bufio.ts";

const positions = [];
const scores = [];
let dice = -1;

for await (const line of readLines(Deno.stdin)) {
  const [, p] = line.split(":");
  positions.push(Number(p));
  scores.push(0);
}

const roll = function () {
  dice += 1;
  return (dice % 100) + 1;
};

const rolls = function (num) {
  return [...new Array(num)].map(roll);
};

const position = function (p, rolls) {
  return ((p + rolls.reduce((acc, v) => acc + v, 0)) % 10) || 10;
};

const play = function () {
  while (true) {
    for (let i = 0; i < positions.length; i += 1) {
      const p = position(positions[i], rolls(3));
      scores[i] += p;
      positions[i] = p;

      if (scores[i] >= 1000) {
        return scores;
      }
    }
  }
};

play();
const numRolls = 1 + dice;
const otherScore = scores.find((s) => s < 1000);

console.log(otherScore * numRolls);
