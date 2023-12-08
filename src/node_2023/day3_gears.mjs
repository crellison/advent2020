const getPositionsToCheck = (lineIndex, numStart, numEnd) => {
  return [
    `${lineIndex},${numStart - 1}`, `${lineIndex},${numEnd + 1}`,
    ...Array.from({ length: numEnd - numStart + 3 }).map((_, i) => `${lineIndex - 1},${numStart - 1 + i}`),
    ...Array.from({ length: numEnd - numStart + 3 }).map((_, i) => `${lineIndex + 1},${numStart - 1 + i}`),
  ]
}

/**
 * @param {String} input 
 */
export const part1 = (input) => {
  const splitInput = input.trimEnd().split('\n');
  const symbolSet = new Set();
  splitInput.forEach((line, i) => {
    for (const symbol of line.matchAll(/([^.\d])/g)) {
      symbolSet.add(`${i},${symbol.index}`)
    }
  })
  let partNumberSum = 0;
  splitInput.forEach((line, i) => {
    for (const number of line.matchAll(/(\d+)/g)) {
      const numStart = number.index;
      const numEnd = numStart + number['0'].length - 1
      // checks positions of number itself too, unoptimal
      if (getPositionsToCheck(i, numStart, numEnd).some(val => symbolSet.has(val))) {
        partNumberSum += +number['0'];
      }
    }
  })
  return partNumberSum;
}

/**
 * @param {String} input 
 */
export const part2 = (input) => {
  const splitInput = input.trimEnd().split('\n');
  const gearMap = {};
  splitInput.forEach((line, i) => {
    for (const symbol of line.matchAll(/(\*)/g)) {
      gearMap[`${i},${symbol.index}`] = []
    }
  })
  splitInput.forEach((line, i) => {
    for (const number of line.matchAll(/(\d+)/g)) {
      const numStart = number.index;
      const numEnd = numStart + number['0'].length - 1
      // checks positions of number itself too, unoptimal
      for (const pos of getPositionsToCheck(i, numStart, numEnd)) {
        if (pos in gearMap) {
          gearMap[pos].push(+number['0'])
        }
      }
    }
  })
  return Object.entries(gearMap).reduce((acc, [_, numbers]) => {
    if (numbers.length === 2) {
      acc += numbers[0] * numbers[1];
    }
    return acc
  }, 0);
}
