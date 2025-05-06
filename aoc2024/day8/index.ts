
interface Vec2 {
  x: number;
  y: number;
}

const run = async () => {
  const input = (await Bun.file("input.txt").text()).trimEnd();
  let count = 0;
  const map = input.split("\n").map((x) => x.split(""));
  const fields: Record<string, Vec2[]> = {};

  // Parse the input and categorize antennas by frequency
  for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[y].length; x++) {
      const char = map[y][x];
      if (char === ".") continue;
      count++;
      fields[char] = fields[char] || [];
      fields[char].push({ x, y });
    }
  }

  const fieldKeys = Object.keys(fields);

  // Loop through all pairs of antennas of the same frequency
  for (const fieldKey of fieldKeys) {
    const antennaPositions = fields[fieldKey];

    for (let i = 0; i < antennaPositions.length; i++) {
      for (let j = i + 1; j < antennaPositions.length; j++) {
        const antenna1 = antennaPositions[i];
        const antenna2 = antennaPositions[j];

        // Calculate the direction vector between the two antennas
        const dx = antenna2.x - antenna1.x;
        const dy = antenna2.y - antenna1.y;

        // Calculate potential antinodes based on the 2:1 rule
        const antinode1: Vec2 = { x: antenna1.x - dx, y: antenna1.y - dy };
        const antinode2: Vec2 = { x: antenna2.x + dx, y: antenna2.y + dy };

        // Place '#' at valid antinode positions within the map bounds
        if (isValidPosition(antinode1, map)) {
          map[antinode1.y][antinode1.x] = "#";
        }
        if (isValidPosition(antinode2, map)) {
          map[antinode2.y][antinode2.x] = "#";
        }
      }
    }
  }
  
  console.log(map.map((row) => row.join("")).join("\n"))
  console.log(map.map((row) => row.join("")).join("\n").split("#").length-1);
};

// Helper function to check if a position is within the map bounds
function isValidPosition(pos: Vec2, map: string[][]): boolean {
  return pos.y >= 0 && pos.y < map.length && pos.x >= 0 && pos.x < map[0].length;
}

run();
