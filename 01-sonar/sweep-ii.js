import { readLines } from "https://deno.land/std/io/bufio.ts";

const currentWindow = [];
let lastMeasurement = null;
let depthIncreases = 0;

const sum = function (iterable) {
  return iterable.reduce((acc, v) => acc + v, 0);
};

for await (const line of readLines(Deno.stdin)) {
  const currentMeasurement = Number(line);
  currentWindow.push(currentMeasurement);

  if (currentWindow.length >= 3) {
    const currentMeasurement = sum(currentWindow);
    if (lastMeasurement && lastMeasurement < currentMeasurement) {
      depthIncreases += 1;
    }
    lastMeasurement = currentMeasurement;
    currentWindow.shift();
  }
}

console.log(depthIncreases);
