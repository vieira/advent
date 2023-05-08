import { readLines } from "https://deno.land/std/io/bufio.ts";

let cuboids = [];

const intersection = function ([xx1, yy1, zz1], [xx2, yy2, zz2]) {
  const [xs1, xe1] = xx1;
  const [xs2, xe2] = xx2;
  const [xs3, xe3] = [Math.max(xs1, xs2), Math.min(xe1, xe2)];
  if (xe3 - xs3 < 0) return [];

  const [ys1, ye1] = yy1;
  const [ys2, ye2] = yy2;
  const [ys3, ye3] = [Math.max(ys1, ys2), Math.min(ye1, ye2)];
  if (ye3 - ys3 < 0) return [];

  const [zs1, ze1] = zz1;
  const [zs2, ze2] = zz2;
  const [zs3, ze3] = [Math.max(zs1, zs2), Math.min(ze1, ze2)];
  if (ze3 - zs3 < 0) return [];

  return [[xs3, xe3], [ys3, ye3], [zs3, ze3]];
};

const split = function ([xx1, yy1, zz1], [xx2, yy2, zz2]) {
  const isect = intersection([xx1, yy1, zz1], [xx2, yy2, zz2]);
  if (!isect.length) {
    return false;
  }

  const [xx3, yy3, zz3] = isect;
  const [xs1, xe1] = xx1;
  const [ys1, ye1] = yy1;
  const [zs1, ze1] = zz1;
  const [xs3, xe3] = xx3;
  const [ys3, ye3] = yy3;
  const [zs3, ze3] = zz3;

  // left
  const cxx1 = [xs1, xs3 - 1];
  const cyy1 = [ys3, ye3];
  const czz1 = [zs3, ze3];

  // right
  const cxx2 = [xe3 + 1, xe1];
  const cyy2 = [ys3, ye3];
  const czz2 = [zs3, ze3];

  // up
  const cxx3 = [xs1, xe1];
  const cyy3 = [ys1, ys3 - 1];
  const czz3 = [zs3, ze3];

  // down
  const cxx4 = [xs1, xe1];
  const cyy4 = [ye3 + 1, ye1];
  const czz4 = [zs3, ze3];

  // top
  const cxx5 = [xs1, xe1];
  const cyy5 = [ys1, ye1];
  const czz5 = [ze3 + 1, ze1];

  // bottom
  const cxx6 = [xs1, xe1];
  const cyy6 = [ys1, ye1];
  const czz6 = [zs1, zs3 - 1];

  return [
    [cxx1, cyy1, czz1],
    [cxx2, cyy2, czz2],
    [cxx3, cyy3, czz3],
    [cxx4, cyy4, czz4],
    [cxx5, cyy5, czz5],
    [cxx6, cyy6, czz6],
  ].filter((coords) => coords.every(([start, end]) => end - start >= 0));
};

const volume = function (coords) {
  return coords.reduce((acc, [start, end]) => (
    acc * (1 + Math.abs(end - start))
  ), 1);
};

const handle = function (action, coords) {
  const scuboids = [];

  for (let i = 0; i < cuboids.length; i += 1) {
    const fracs = split(cuboids[i], coords);
    if (!fracs) {
      scuboids.push(cuboids[i]);
    }

    if (fracs.length) {
      fracs.forEach((f) => scuboids.push(f));
    }
  }

  if (action === "on") scuboids.push(coords);
  cuboids = scuboids;
};

for await (const line of readLines(Deno.stdin)) {
  const [action, coords] = line.split(" ");
  const [xx, yy, zz] = coords.split(",").map((c) => {
    const [_, interval] = c.split("=");
    const [start, end] = interval.split("..").map(Number);
    if (start < -50 && end < -50 || start > 50 && end > 50) return false;
    return [Math.max(-50, start), Math.min(50, end)];
  });
  if ([xx, yy, zz].includes(false)) continue;
  handle(action, [xx, yy, zz]);
}

console.log(cuboids.reduce((acc, c) => acc + volume(c), 0));
