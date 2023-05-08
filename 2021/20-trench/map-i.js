import { readLines } from "https://deno.land/std/io/bufio.ts";

let algo = "";
let image = [];

for await (const line of readLines(Deno.stdin)) {
  if (!image.length && line.length) algo += line;
  else image.push(line.split(""));
}

image = image.filter((l) => l.length);
let bg = ".";

const adjacents = function (x, y) {
  const top = [x, y - 1];
  const bottom = [x, y + 1];
  const left = [x - 1, y];
  const middle = [x, y];
  const right = [x + 1, y];
  const topLeft = [x - 1, y - 1];
  const topRight = [x + 1, y - 1];
  const bottomLeft = [x - 1, y + 1];
  const bottomRight = [x + 1, y + 1];
  return [
    topLeft,
    top,
    topRight,
    left,
    middle,
    right,
    bottomLeft,
    bottom,
    bottomRight,
  ];
};

const readBlock = function (x, y) {
  return Number.parseInt(
    adjacents(x, y)
      .map(([x, y]) => (image[y]?.[x] ?? bg) === "#" ? 1 : 0)
      .join(""),
    2,
  );
};

const calcPixel = function (x, y) {
  return algo[readBlock(x, y)];
};

const invertBackground = function () {
  bg = bg === "." ? "#" : ".";
};

const enhanceImage = function () {
  const img = [];
  for (let y = 0; y <= image.length + 1; y += 1) {
    img.push([]);
    const row = image[0];

    for (let x = 0; x <= row.length + 1; x += 1) {
      img[y][x] = calcPixel(x - 1, y - 1);
    }
  }

  if (algo.startsWith("#")) invertBackground();

  return img;
};

const countLitPixels = function (img) {
  return img.reduce((acc, row) => (
    acc + row.filter((px) => px === "#").length
  ), 0);
};

for (let i = 0; i < 2; i += 1) {
  image = enhanceImage();
}
console.log(countLitPixels(image));
