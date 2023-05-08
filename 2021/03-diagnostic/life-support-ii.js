import { readLines } from "https://deno.land/std/io/bufio.ts";

const readings = [];

const handleReading = function (reading) {
  readings.push(reading.map((r) => +r));
};

const mostCommonBit = function (rs, pos) {
  const len = rs.length;
  const bitCount = rs.filter((r) => r[pos]).length;
  return Math.round(bitCount / len);
};

const filterByBit = function ({ criteria }) {
  let remaining = readings;

  let i = 0;
  while (true) {
    if (remaining.length <= 1) {
      const [reading] = remaining;
      return Number.parseInt(reading.join(""), 2);
    }

    const bit = Math.abs(criteria - mostCommonBit(remaining, i));
    remaining = remaining.filter((reading) => reading[i] === bit);
    i += 1;
  }
};

const rateO2Gen = function () {
  const mostCommonBit = 0;
  return filterByBit({ criteria: mostCommonBit });
};

const rateCO2Scrub = function () {
  const leastCommonBit = 1;
  return filterByBit({ criteria: leastCommonBit });
};

for await (const line of readLines(Deno.stdin)) {
  handleReading(line.split(""));
}

const O2GenRating = rateO2Gen();
const CO2ScrubRating = rateCO2Scrub();

console.log(O2GenRating * CO2ScrubRating);
