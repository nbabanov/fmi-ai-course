import * as readline from 'node:readline';
import { performance } from 'node:perf_hooks';

const askQuestion = (query: string, answerLines = 1): Promise<string> => {
  const input: string[] = [];

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  console.log(query);
  rl.prompt();

  return new Promise(
    (resolve) => {
      rl.on('line', (line) => {
        input.push(line);
        if (input.length == answerLines) {
          // resolve(input.join('\n'));
          rl.close();
        }
      });

      rl.on('close', () => {
        resolve(input.join('\n'));
      });
    }
  );
};

interface Point2D {
  x: number;
  y: number;
}

const getManhattenDistance = (a: Point2D, b: Point2D): number => {
  return Math.floor(Math.abs(a.x - b.x)) + Math.floor(Math.abs(a.y - b.y));
};

const defaultStepCost = (boardSide: number, a: PuzzleNode, b: PuzzleNode): number =>
  getManhattenDistance(
    boardToPoint2D(boardSide, a.board),
    boardToPoint2D(boardSide, b.board)
  );

interface PuzzleNode {
  cost: number;
  board: number[];
  pathToPuzzleNode: string[];
}
type Heuristic = (root: PuzzleNode) => number;
type GoalPredicate = (node: PuzzleNode) => boolean;

interface SearchResult {
  cost: number;
  node: PuzzleNode | null;
}

enum BoardMove {
  Up,
  Right,
  Down,
  Left,
}

const boardMoveToString = (move: BoardMove): string => {
  switch (move) {
    case BoardMove.Up:
      return 'up';
    case BoardMove.Right:
      return 'right';
    case BoardMove.Down:
      return 'down';
    case BoardMove.Left:
      return 'left';
  }
};

const getMoveIndex = (
  boardSide: number,
  currentIndex: number,
  move: BoardMove
): number => {
  switch (move) {
    case BoardMove.Down:
      return currentIndex - boardSide;
    case BoardMove.Up:
      return currentIndex + boardSide;
    case BoardMove.Right:
      if (currentIndex % boardSide == 0) {
        return -1;
      }
      return currentIndex - 1;
    case BoardMove.Left:
      if (currentIndex % boardSide == boardSide - 1) {
        return boardSide * boardSide;
      }

      return currentIndex + 1;
  }
};

const getZeroIndex = (array: number[]): number => array.indexOf(0);

const getSuccessors = (boardSide: number, node: PuzzleNode): PuzzleNode[] => {
  const zeroIndex = getZeroIndex(node.board);
  const boardLength = node.board.length;

  const result: PuzzleNode[] = [];

  for (let i = 0; i < 4; i++) {
    const moveIndex = getMoveIndex(boardSide, zeroIndex, i as BoardMove);

    if (moveIndex >= 0 && moveIndex < boardLength) {
      const board = [...node.board];
      const path = [...node.pathToPuzzleNode];
      const boardMoveString = boardMoveToString(i as BoardMove);

      path.push(boardMoveString);
      board[zeroIndex] = board[moveIndex];
      board[moveIndex] = 0;
      result.push({
        cost: defaultStepCost(boardSide, node, { cost: -1, board, pathToPuzzleNode: [] }), // TODO: Use manhatten to calculate cost!
        board,
        pathToPuzzleNode: path,
      });
    }
  }

  return result;
};

type StepCost = (a: PuzzleNode, b: PuzzleNode) => number;

const isNodeInPath = (node: PuzzleNode, path: PuzzleNode[]): boolean =>
  path.find(
    (element) => JSON.stringify(element?.board) === JSON.stringify(node?.board)
  ) != null;

const search = (
  boardSide: number,
  heuristic: Heuristic,
  stepCost: StepCost,
  isGoal: GoalPredicate,
  path: PuzzleNode[],
  currentCost: number,
  threshold: number
): SearchResult => {
  const node = path[path.length - 1];
  const cheapestPathCost = currentCost + heuristic(node);

  if (cheapestPathCost > threshold) {
    return {
      cost: cheapestPathCost,
      node: null,
    };
  }

  if (isGoal(node)) {
    return { cost: cheapestPathCost, node };
  }

  let min = Number.MAX_SAFE_INTEGER;

  const successors = getSuccessors(boardSide, node);

  for (const successor of successors) {
    const isInPath = isNodeInPath(successor, path);

    if (!isInPath) {
      path.push(successor);
      const searchResult = search(
        boardSide,
        heuristic,
        stepCost,
        isGoal,
        path,
        currentCost + stepCost(node, successor),
        threshold
      );

      if (searchResult.node != null) {
        return searchResult;
      }

      if (searchResult.cost < min) {
        min = searchResult.cost;
      }

      path.pop();
    }
  }

  return { cost: min, node: null };
};

interface IDAResult {
  path: string[];
  threshold: number;
}

const idaStar = (
  boardSide: number,
  heuristic: Heuristic,
  stepCost: StepCost,
  isGoal: GoalPredicate,
  root: PuzzleNode,
  isDebug = false
): IDAResult | null => {
  const minThreshold = getSuccessors(boardSide, root).map(({ cost }) => cost).sort()[0];
  let threshold = minThreshold;
  const path = [root];

  while (true) {
    const result = search(
      boardSide,
      heuristic,
      stepCost,
      isGoal,
      path,
      0,
      threshold
    );
    if (result && isGoal(result.node as unknown as PuzzleNode)) {
      if (isDebug) {
        path.forEach((node) =>
          console.log(
            node.board.reduce(
              (acc, element) => {
                if (acc.count % boardSide === 0) {
                  acc.result += '\n';
                }
                acc.result += element + ' ';
                acc.count++;

                return acc;
              },
              { count: 0, result: '' }
            ).result
          )
        );
      }

      return { path: result?.node?.pathToPuzzleNode ?? [], threshold };
    }

    if (result && isNodeInPath(result.node, path)) {
      return null;
    }

    threshold = result.cost;
  }
};

const isGoalCreator =
  (goalIndex: number) =>
  (node: PuzzleNode): boolean => {
    if (node == null) {
      return false;
    }

    const goalBoard = [...node.board].sort();
    goalBoard.shift();
    goalBoard.splice(goalIndex, 0, 0);

    return JSON.stringify(goalBoard) === JSON.stringify(node.board);
  };

const boardSizeToBoardSide = (size: number): number =>
  Math.floor(Math.abs(Math.sqrt(size + 1)));

const index1DToIndex2D = (boardSide: number, index: number): Point2D => ({
  x: index % boardSide,
  y: index / boardSide,
});

const boardToPoint2D = (boardSide: number, board: number[]): Point2D =>
  index1DToIndex2D(boardSide, getZeroIndex(board));

const solvePuzzle = (
  boardSize: number,
  goalIndexRaw: number,
  board: number[]
): void => {
  const goalIndex = goalIndexRaw === -1 ? boardSize : goalIndexRaw;
  const boardSide = boardSizeToBoardSide(boardSize);
  const isGoal = isGoalCreator(goalIndex);
  const heuristic: Heuristic = (node: PuzzleNode): number =>
    getManhattenDistance(
      boardToPoint2D(boardSide, node.board),
      index1DToIndex2D(boardSide, goalIndex)
    );

  const stepCost: StepCost = (a: PuzzleNode, b: PuzzleNode): number =>
      defaultStepCost(boardSide, a, b);

  const root: PuzzleNode = { cost: 0, board, pathToPuzzleNode: [] };

  const idaStart = performance.now();
  const result = idaStar(boardSide, heuristic, stepCost, isGoal, root);
  const idaEnd = performance.now();

  console.log(result?.threshold);
  console.log(result?.path.join('\n'));
  console.log(`IDA* took: ${(idaEnd - idaStart) / 1000}s`);
};

const isSolvable = (boardSize: number, board: number[]): boolean => {
  let inversions = 0;

  for (let i =0; i <= boardSize; i++) {
      for (let j = i + 1; j <= boardSize; j++) {
          if (board[j] > board[i]){
              inversions++;
          }
      }
  }

  return inversions % 2 === 0;
}

const main = async () => {
  const boardSize = Number(await askQuestion('What is the board size (N)?'));
  const goalIndex = Number(await askQuestion('Where is the goal?'));
  const boardRaw = await askQuestion('What is the initial board?', boardSizeToBoardSide(boardSize));
  const initialBoard = boardRaw
                          .replace(/\n/g, ' ')
                          .split(' ')
                          .map(Number);

  if (!isSolvable(boardSize, initialBoard)) {
    console.error('The given board is not solvable!');
    process.exit();
  }
  
  solvePuzzle(boardSize, goalIndex, initialBoard);
};

main();
