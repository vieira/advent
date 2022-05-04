import { readLines } from "https://deno.land/std/io/bufio.ts";

const position = { x: 0, y: 0, aim: 0 };

const handleCommand = function ([cmd, d]) {
  const distance = Number(d);

  switch (cmd) {
    case "forward":
      position.x += distance;
      position.y += position.aim * distance;
      break;
    case "down":
      position.aim += distance;
      break;
    case "up":
      position.aim -= distance;
      break;
  }
};

for await (const line of readLines(Deno.stdin)) {
  handleCommand(line.split(" "));
}

console.log(position.x * position.y);
