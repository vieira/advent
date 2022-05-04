import { readLines } from "https://deno.land/std/io/bufio.ts";

const blocks = [];

class Process {
  #memory;
  #input = [];
  #instructions = [];

  constructor(instructions, memory) {
    this.#instructions = instructions;
    this.#memory = new Map(memory);
  }

  #value(arg) {
    return Number.isInteger(+arg) ? +arg : this.#memory.get(arg);
  }

  inp(name) {
    this.#memory.set(name, this.#input.shift());
  }

  add(arg1, arg2) {
    this.#memory.set(arg1, this.#value(arg1) + this.#value(arg2));
  }

  mul(arg1, arg2) {
    this.#memory.set(arg1, this.#value(arg1) * this.#value(arg2));
  }

  div(arg1, arg2) {
    this.#memory.set(arg1, Math.floor(this.#value(arg1) / this.#value(arg2)));
  }

  mod(arg1, arg2) {
    this.#memory.set(arg1, this.#value(arg1) % this.#value(arg2));
  }

  eql(arg1, arg2) {
    this.#memory.set(arg1, Number(this.#value(arg1) === this.#value(arg2)));
  }

  run(input) {
    this.#input = input.toString().split("").map(Number);
    this.#instructions.forEach(([op, arg1, arg2]) => this[op](arg1, arg2));
    return this.#memory;
  }
}

const successors = function* (state) {
  const { candidate, number } = state;
  if (number.length >= blocks.length) return [];

  for (let w = 9; w > 0; w -= 1) {
    const process = new Process(
      blocks[number.length],
      [["x", 0], ["y", 0], ["w", 0], ["z", candidate.z]],
    );
    const out = process.run(w);
    const z = out.get("z");
    yield ({ candidate: { z, w }, number: number + w });
  }
};

const isGoal = function (state) {
  const { number, candidate } = state;
  if (number.length < blocks.length) return false;
  return candidate.z === 0;
};

const findAssignments = function () {
  const ss = [{ candidate: { z: 0 }, number: "" }];
  const visited = Array.from(Array(blocks.length), () => new Set());

  while (ss.length) {
    const current = ss.pop();
    if (isGoal(current)) {
      return current;
    }
    for (const s of successors(current)) {
      const { number, candidate } = s;
      const blid = number.length - 1;
      if (visited[blid].has(candidate.z)) continue;
      visited[blid].add(candidate.z);
      ss.push(s);
    }
  }
};

for await (const line of readLines(Deno.stdin)) {
  const instruction = line.split(" ");
  const [op] = instruction;
  if (op === "inp") {
    blocks.push([instruction]);
  } else {
    blocks[blocks.length - 1].push(instruction);
  }
}

const { number } = findAssignments();
console.log(number);
