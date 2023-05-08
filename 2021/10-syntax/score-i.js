import { readLines } from "https://deno.land/std/io/bufio.ts";

let score = 0;
const closers = new Map([
  [")", { opener: "(", score: 3 }],
  ["]", { opener: "[", score: 57 }],
  ["}", { opener: "{", score: 1197 }],
  [">", { opener: "<", score: 25137 }],
]);

const checkSyntax = function (line) {
  const stack = [];
  for (const char of line.split("")) {
    if (closers.has(char)) {
      const opener = stack.pop();
      const closer = closers.get(char);
      if (opener !== closer.opener) return closer.score;
    } else {
      stack.push(char);
    }
  }
  return 0;
};

for await (const line of readLines(Deno.stdin)) {
  score += checkSyntax(line);
}

console.log(score);
