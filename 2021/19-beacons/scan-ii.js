import { readLines } from "https://deno.land/std/io/bufio.ts";

const scanners = new Map();
let scannerId = 0;

for await (const line of readLines(Deno.stdin)) {
  if (!line) continue;
  if (line.startsWith("---")) {
    const [, , id] = line.split(" ");
    scannerId = Number(id);
    scanners.set(scannerId, []);
    continue;
  }
  const coordinates = line.split(",").map(Number);
  scanners.get(scannerId).push(coordinates);
}

const difference = function (p1, p2) {
  return p1.map((v, i) => v - p2[i]);
};

const sum = function (p1, p2) {
  return p1.map((v, i) => v + p2[i]);
};

const eq = function (p1, p2) {
  return p1.every((v, i) => (
    v === p2[i] && Math.sign(1 / v) === Math.sign(1 / p2[i])
  ));
};

const rotate = function (v, r) {
  return v.map((_, i) => Math.sign(1 / r[i]) * v[Math.abs(r[i])]);
};

const arrangements = function (bs) {
  return [
    [0, 1, 2],
    [0, -1, -2],
    [0, 2, -1],
    [0, -2, 1],
    [-0, 1, -2],
    [-0, -1, 2],
    [-0, 2, 1],
    [-0, -2, -1],
    [1, 0, -2],
    [1, -0, 2],
    [1, 2, 0],
    [1, -2, -0],
    [-1, 0, 2],
    [-1, -0, -2],
    [-1, 2, -0],
    [-1, -2, 0],
    [2, 0, 1],
    [2, -0, -1],
    [2, 1, -0],
    [2, -1, 0],
    [-2, 0, -1],
    [-2, -0, 1],
    [-2, 1, 0],
    [-2, -1, -0],
  ].map((rotation) => (
    { rotation, beacons: bs.map((b) => rotate(b, rotation)) }
  ));
};

const intersection = function (s1, s2, overlaps = 12) {
  const distances = new Map();
  for (let i = 0; i < s1.length; i += 1) {
    for (let j = 0; j < s2.length; j += 1) {
      const d = difference(s1[i], s2[j]);
      const dkey = d.join(",");
      const ovlps = (distances.get(dkey) ?? []);
      ovlps.push([s1[i], s2[j]]);
      distances.set(dkey, ovlps);

      if (ovlps.length >= overlaps) {
        return [dkey.split(",").map(Number), ovlps];
      }
    }
  }
};

const anysection = function (s1, s2, overlaps = 12) {
  const a1 = { beacons: s1, rotation: [0, 1, 2] };
  for (const a2 of arrangements(s2)) {
    const intr = intersection(a1.beacons, a2.beacons, overlaps);
    if (!intr) continue;
    const { rotation } = a2;
    const [d, coordinates] = intr;
    const distance = d;
    return {
      distance,
      rotation,
      coordinates: coordinates.map(([c1, c2]) => [c1, c2]),
    };
  }
};

const manhattanDistance = function (d1, d2) {
  return d1.reduce((acc, di, i) => acc + Math.abs(di - d2[i]), 0);
};

const maxManhattanDistance = function (distances) {
  let max = 0;

  for (let i = 1; i <= distances.size; i += 1) {
    for (let j = i + 1; j <= distances.size; j += 1) {
      const d = manhattanDistance(distances.get(i), distances.get(j));
      max = Math.max(d, max);
    }
  }

  return max;
};

const map = function () {
  const distances = new Map();
  const refScanner = scanners.get(0);

  while (scanners.size > 1) {
    for (const i of scanners.keys()) {
      const rmtScanner = scanners.get(i);
      if (i === 0) continue;
      const intr = anysection(refScanner, rmtScanner);
      if (!intr) continue;
      const { distance, rotation } = intr;
      distances.set(i, distance);
      rmtScanner.forEach((coordinate) => {
        const translated = sum(rotate(coordinate, rotation), distance);
        if (!refScanner.find((c) => eq(c, translated))) {
          refScanner.push(translated);
        }
      });
      scanners.delete(i);
    }
  }
  return distances;
};

console.log(maxManhattanDistance(map()));
