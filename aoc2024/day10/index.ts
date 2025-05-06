
type Position = [number, number];

async function parseMap(): Promise<number[][]> {
  return (await Bun.file("input.txt").text()) 
    .split("\n")
    .map((line) => line.split("").map(Number));
}

function findTrailheads(map: number[][]): Position[] {
  const trailheads: Position[] = [];
  for (let row = 0; row < map.length; row++) {
    for (let col = 0; col < map[0].length; col++) {
      if (map[row][col] === 0) {
        trailheads.push([row, col]);
      }
    }
  }
  return trailheads;
}

function isValidPosition(map: number[][], row: number, col: number): boolean {
  return row >= 0 && col >= 0 && row < map.length && col < map[0].length;
}

function bfsTrailhead(map: number[][], start: Position): number {
  const directions: Position[] = [
    [0, 1],  // Right
    [1, 0],  // Down
    [0, -1], // Left
    [-1, 0], // Up
  ];

  const queue: Position[] = [start];
  const visited = new Set<string>();
  const reachableNines = new Set<string>();

  while (queue.length > 0) {
    const [row, col] = queue.shift()!;
    const key = `${row},${col}`;

    if (visited.has(key)) continue;
    visited.add(key);

    // Check if the current position is a 9
    if (map[row][col] === 9) {
      reachableNines.add(key);
      continue;
    }

    // Explore neighbors
    for (const [dr, dc] of directions) {
      const newRow = row + dr;
      const newCol = col + dc;
      if (
        isValidPosition(map, newRow, newCol) &&
        map[newRow][newCol] === map[row][col] + 1
      ) {
        queue.push([newRow, newCol]);
      }
    }
  }

  return reachableNines.size;
}

function calculateScores(map: number[][]): number {
  const trailheads = findTrailheads(map);
  let totalScore = 0;

  for (const trailhead of trailheads) {
    totalScore += bfsTrailhead(map, trailhead);
  }

  return totalScore;
}

// Example Usage
console.time("execution time")
const map = await parseMap();
const result = calculateScores(map);

console.log(`Total sum of scores of all trailheads: ${result}`);
console.timeEnd("execution time");
