import { readLines } from "https://deno.land/std/io/bufio.ts";

const position = [0, 0];

const handleCommand = function ([cmd, d]) {
  const distance = Number(d);

  switch (cmd) {
    case "forward":
      position[0] += distance;
      break;
    case "down":
      position[1] += distance;
      break;
    case "up":
      position[1] -= distance;
      break;
  }
};

for await (const line of readLines(Deno.stdin)) {
  handleCommand(line.split(" "));
}

console.log(position.reduce((acc, v) => acc * v, 1));
