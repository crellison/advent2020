/**
 * @param {String} input 
 */
export const part1 = (input) => {
  let lottoWinnings = 0;
  input.trimEnd().split('\n').forEach(line => {
    const colonPos = line.indexOf(':');
    const barPos = line.indexOf('|');
    const winningNums = [...line.substring(colonPos, barPos).matchAll(/\d+/g)].map(e => e['0']);
    const cardNums = [...line.substring(barPos).matchAll(/\d+/g)].map(e => e['0']);

    const matchingNums = winningNums.filter(val => cardNums.includes(val));
    if (matchingNums.length)
      lottoWinnings += Math.pow(2, matchingNums.length - 1);
  })
  return lottoWinnings;
}

/**
 * @param {String} input 
 */
export const part2 = (input) => {
  const splitInput = input.trimEnd().split('\n');
  const cardCounts = Array.from({ length: splitInput.length }).fill(1);
  splitInput.forEach((line, i) => {
    const colonPos = line.indexOf(':');
    const barPos = line.indexOf('|');
    const winningNums = [...line.substring(colonPos, barPos).matchAll(/\d+/g)].map(e => e['0']);
    const cardNums = [...line.substring(barPos).matchAll(/\d+/g)].map(e => e['0']);
    const matchingNums = winningNums.filter(val => cardNums.includes(val));

    for (let offset = 1; offset <= matchingNums.length; offset++) {
      if (i + offset < cardCounts.length) {
        cardCounts[i + offset] += cardCounts[i]
      }
    }
  })
  return cardCounts.reduce((acc, curr) => acc + curr);
}
