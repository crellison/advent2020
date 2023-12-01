/**
 * @param {String} input 
 */
export const part1 = (input) => {
  return input.split('\n').reduce((acc, line) => {
    if (line.length === 0) {
      return acc
    }
    let startCharIndex = 0, endCharIndex = line.length - 1;

    while (isNaN(+line[startCharIndex])) {
      startCharIndex++;
      if (startCharIndex > line.length) {
        throw new RangeError(`could not find numeric char in line: ${line}`);
      }
    }
    while (isNaN(+line[endCharIndex])) {
      endCharIndex--;
      if (endCharIndex < 0) {
        throw new RangeError(`could not find numeric char in line: ${line}`);
      }
    }

    return acc + Number(`${line[startCharIndex]}${line[endCharIndex]}`);
  }, 0)
}

const numbers = [
  ['one', 1],
  ['two', 2],
  ['three', 3],
  ['four', 4],
  ['five', 5],
  ['six', 6],
  ['seven', 7],
  ['eight', 8],
  ['nine', 9],
]

/**
 * @param {String} input 
 */
export const part2 = (input) => {
  return input.split('\n').reduce((acc, line) => {
    if (line.length === 0) {
      return acc
    }

    let startCharIndex = 0, endCharIndex = line.length - 1;
    let firstDigit = NaN, lastDigit = NaN;

    startSearch: while (startCharIndex < line.length) {
      if (!isNaN(+line[startCharIndex])) {
        firstDigit = line[startCharIndex];
        break startSearch;
      }

      for (const [num, digit] of numbers) {
        if (line.startsWith(num, startCharIndex)) {
          firstDigit = digit;
          break startSearch;
        }
      }
      startCharIndex++;
    }

    endSearch: while (endCharIndex >= 0) {
      if (!isNaN(+line[endCharIndex])) {
        lastDigit = line[endCharIndex];
        break endSearch;
      }

      for (const [num, digit] of numbers) {
        if (line.endsWith(num, endCharIndex + 1)) {
          lastDigit = digit;
          break endSearch;
        }
      }
      endCharIndex--;
    }

    if (isNaN(firstDigit) || isNaN(lastDigit)) {
      throw new RangeError(`failed to find digits in ${line}: ${firstDigit} & ${lastDigit}`);
    }

    return acc + Number(`${firstDigit}${lastDigit}`);
  }, 0)
}
