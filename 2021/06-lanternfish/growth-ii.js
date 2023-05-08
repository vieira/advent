import { readLines } from "https://deno.land/std/io/bufio.ts";

const state = new Map([...Array(9)].map((_, t) => [t, 0]));

for await (const line of readLines(Deno.stdin)) {
  line.split(",").map((t) => +t).forEach((t) => {
    state.set(t, state.get(t) + 1);
  });
}

const simulateGrowth = function (numDays) {
  for (let day = 0; day < numDays; ++day) {
    const numReproductions = state.get(0);
    for (let t = 0; t < state.size - 1; ++t) {
      state.set(t, state.get(t + 1));
    }
    state.set(6, state.get(6) + numReproductions);
    state.set(8, numReproductions);
  }
};

const countPopulation = function () {
  let population = 0;
  state.forEach((count) => {
    population += count;
  });
  return population;
};

simulateGrowth(256);
console.log(countPopulation());
