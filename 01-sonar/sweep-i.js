import { readLines } from "https://deno.land/std/io/bufio.ts";

let lastMeasurement = null;
let depthIncreases = 0;

for await (const line of readLines(Deno.stdin)) {
  const currentMeasurement = Number(line);
  if (lastMeasurement && lastMeasurement < currentMeasurement) {
    depthIncreases += 1;
  }
  lastMeasurement = currentMeasurement;
}

console.log(depthIncreases);
