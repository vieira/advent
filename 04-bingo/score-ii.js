import { readLines } from "https://deno.land/std/io/bufio.ts";

let numbers;
let boards = [];
const winners = [];

const readInput = (() => {
  let lineNo = -1;
  let boardNo = -1;

  return (line) => {
    lineNo += 1;
    if (lineNo === 0) {
      numbers = line.split(",").map((n) => +n);
      return;
    }
    if (((lineNo - 1) % 6) === 0) {
      boardNo += 1;
      boards.push([]);
      return;
    }

    boards[boardNo].push(line.trim().split(/\s+/).map((n) => +n));
  };
})();

const hasRow = function (board) {
  return !!board.find((row) => row.every((position) => position === true));
};

const hasColumn = function (board) {
  return !!board[0].find((num, i) => (
    num === true && board.every((row) => row[i] === true)
  ));
};

const markBoard = function (board, num) {
  return board.map((row) => (
    row.map((k) => num === k || k)
  ));
};

const drawNumber = function (num) {
  const roundWinners = [];
  boards.forEach((board, i) => {
    boards[i] = markBoard(board, num);
    if (hasRow(boards[i]) || hasColumn(boards[i])) {
      winners.push(boards[i]);
      roundWinners.push(i);
    }
  });
  return boards.filter((_, i) => !roundWinners.includes(i));
};

const computeScore = function (board, num) {
  return num * board
    .flatMap((num) => num)
    .filter((num) => num !== true)
    .reduce((acc, v) => acc + v, 0);
};

for await (const line of readLines(Deno.stdin)) {
  readInput(line);
}

const lastNum = numbers.find((num) => {
  boards = drawNumber(num);
  return !boards.length;
});
const lastBoard = winners[winners.length - 1];
console.log(computeScore(lastBoard, lastNum));
