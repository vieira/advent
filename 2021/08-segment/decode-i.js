import { readLines } from "https://deno.land/std/io/bufio.ts";

const signals = [];

for await (const line of readLines(Deno.stdin)) {
  const [, output] = line.split(" | ");
  signals.push(output.split(" "));
}

const lengths = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

const countDigits = function () {
  let counter = 0;
  signals.forEach((signal) => {
    signal.forEach((digit) => {
      const candidates = [];
      lengths.forEach((length, num) => {
        if (length === digit.length) {
          candidates.push(num);
        }
      });

      if (candidates.length === 1) {
        counter += 1;
      }
    });
  });
  return counter;
};

console.log(countDigits());
