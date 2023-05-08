import { readLines } from "https://deno.land/std/io/bufio.ts";

let numReadings = 0;
let readings;

const handleReading = function (reading) {
  if (!readings) {
    readings = Array(reading.length).fill(0);
  }

  numReadings += 1;

  reading.forEach((bit, i) => {
    if (+bit) readings[i] += 1;
  });
};

const computeResults = function () {
  const gammaBits = readings.map((count) => (
    Math.round(count / numReadings)
  ));
  const epsilonBits = gammaBits.map((bit) => Math.abs(bit - 1));
  const gamma = Number.parseInt(gammaBits.join(""), 2);
  const epsilon = Number.parseInt(epsilonBits.join(""), 2);
  return [gamma, epsilon];
};

for await (const line of readLines(Deno.stdin)) {
  handleReading(line.split(""));
}

const [gamma, epsilon] = computeResults();
console.log(gamma * epsilon);
