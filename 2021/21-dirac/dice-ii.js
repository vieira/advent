import { readLines } from "https://deno.land/std/io/bufio.ts";

const positions = [];
const rolls = [
  [3, 1],
  [4, 3],
  [5, 6],
  [6, 7],
  [7, 6],
  [8, 3],
  [9, 1],
];
const memo = new Map();

for await (const line of readLines(Deno.stdin)) {
  const [, p] = line.split(":");
  positions.push(+p);
}

const position = function (p, rolls) {
  return ((p + rolls) % 10) || 10;
};

const hasWon = function (score) {
  return score >= 21;
};

const findWinner = function ({ scores }) {
  return scores.findIndex(hasWon);
};

const sumWins = function (w1, w2) {
  return w1.map((v1, i) => v1 + w2[i]);
};

const play = function (state, d) {
  const winner = findWinner(state);

  if (winner > -1) {
    const w = [0, 0];
    w[winner] = state.universes;
    return w;
  }

  const { positions, scores, universes, wins } = state;
  const key = `${positions.join(",")};${scores.join(",")}`;

  if (memo.has(key)) {
    return memo.get(key).map((w) => w * universes);
  }

  for (const [r1, u1] of rolls) {
    const p1 = position(positions[0], r1);
    for (const [r2, u2] of rolls) {
      const nextState = {
        positions: [...positions],
        scores: [...scores],
        wins: [...wins],
        universes,
      };
      nextState.positions[0] = p1;
      nextState.scores[0] += p1;
      nextState.universes *= u1;
      const [s1] = nextState.scores;
      if (!hasWon(s1)) {
        const p2 = position(positions[1], r2);
        nextState.positions[1] = p2;
        nextState.scores[1] += p2;
        nextState.universes *= u2;
      }
      state.wins = sumWins(state.wins, play(nextState, d + 1));
      if (hasWon(s1)) break;
    }
  }
  memo.set(key, state.wins.map((w) => w / state.universes));
  return state.wins;
};

const wins = play({ positions, scores: [0, 0], wins: [0, 0], universes: 1 });
console.log(wins.reduce((acc, v) => v > acc ? v : acc, 0));
