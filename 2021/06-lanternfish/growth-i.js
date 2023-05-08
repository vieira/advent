import { readLines } from "https://deno.land/std/io/bufio.ts";

let state;

for await (const line of readLines(Deno.stdin)) {
  state = line.split(",").map((t) => +t);
}

const simulateGrowth = function (numDays) {
  for (let day = 0; day < numDays; ++day) {
    const newborns = [];
    state = state.map((timer) => {
      if (timer > 0) return timer - 1;
      newborns.push(8);
      return 6;
    });
    state = state.concat(newborns);
  }
};

simulateGrowth(80);
console.log(state.length);
