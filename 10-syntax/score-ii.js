import { readLines } from "https://deno.land/std/io/bufio.ts";

const scores = [];
const closers = new Map([
  [")", { opener: "(", score: 3 }],
  ["]", { opener: "[", score: 57 }],
  ["}", { opener: "{", score: 1197 }],
  [">", { opener: "<", score: 25137 }],
]);

const openers = new Map(
  Array.from(closers).map((
    [k, v],
    i,
  ) => [v.opener, { closer: k, score: i + 1 }]),
);

const checkSyntax = function (line) {
  const stack = [];
  for (const char of line.split("")) {
    if (closers.has(char)) {
      const opener = stack.pop();
      const closer = closers.get(char);
      if (opener !== closer.opener) return false;
    } else {
      stack.push(char);
    }
  }

  return stack.reverse().reduce((acc, char) => (
    acc * 5 + openers.get(char).score
  ), 0);
};

for await (const line of readLines(Deno.stdin)) {
  const score = checkSyntax(line);
  if (score !== false) {
    scores.push(score);
  }
}

console.log(scores.sort((x, y) => x - y)[Math.floor(scores.length / 2)]);
