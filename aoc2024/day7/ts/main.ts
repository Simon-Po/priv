function calculateAllPossibilities(numbers: number[]): number[] {
    function helper(index: number, currentResult: number): number[] {
        // Base case: if we've processed all numbers, return the current result
        if (index === numbers.length) {
            return [currentResult];
        }

        const nextNum = numbers[index];
        // Recursive cases: apply + and * with the next number, respecting left-to-right order
        const addResults = helper(index + 1, currentResult + nextNum);
        const multiplyResults = helper(index + 1, currentResult * nextNum);

        // Combine results
        return [...addResults, ...multiplyResults];
    }

    // Edge case: no numbers provided
    if (numbers.length === 0) return [];

    // Start recursion with the first number
    return Array.from(new Set(helper(1, numbers[0]))); // Remove duplicates
}

// Main program logic
const rawInput = await Bun.file("input.txt").text();
const input = rawInput.trimEnd().split("\n").map(line => line.split(":"));
let validSum = 0;

for (const [resultRaw, numbersRaw] of input) {
    const result = Number(resultRaw);
    const numbers = numbersRaw.trim().split(" ").map(Number);

    // Calculate all possible results for this set of numbers
    const possibilities = calculateAllPossibilities(numbers);

    // If the result is in possibilities, add it to the sum
    if (possibilities.includes(result)) {
        validSum += result;
    }
}

console.log(validSum);
export {}
