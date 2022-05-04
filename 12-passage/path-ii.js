import { readLines } from "https://deno.land/std/io/bufio.ts";

const caves = new Map();

const addPath = function (start, end) {
  if (!caves.has(start)) {
    caves.set(start, new Set([end]));
    return;
  }
  caves.get(start).add(end);
};

const adjacents = function (cave) {
  return caves.get(cave) ?? new Set();
};

const isSmall = function (cave) {
  return cave === cave.toLowerCase();
};

const isGoal = function (cave) {
  return cave === "end";
};

const isStart = function (cave) {
  return cave === "start";
};

const findAllPaths = function () {
  const paths = new Set();
  const successors = [{ cave: "start", path: [], canRepeatVisit: true }];

  while (successors.length) {
    const { cave, path, canRepeatVisit } = successors.pop();
    if (isGoal(cave)) {
      paths.add([...path, cave].join("-"));
      continue;
    }

    const isSmallVisitedCave = isSmall(cave) && path.includes(cave);
    let hasUsedRepeatVisit = !canRepeatVisit;
    if (isSmallVisitedCave) {
      if (!canRepeatVisit || isStart(cave)) {
        continue;
      }
      hasUsedRepeatVisit = true;
    }

    adjacents(cave).forEach((nextCave) => {
      successors.push({
        cave: nextCave,
        path: [...path, cave],
        canRepeatVisit: !hasUsedRepeatVisit,
      });
    });
  }

  return paths;
};

for await (const line of readLines(Deno.stdin)) {
  const [cave1, cave2] = line.split("-");
  addPath(cave1, cave2);
  addPath(cave2, cave1);
}

console.dir(findAllPaths().size);
