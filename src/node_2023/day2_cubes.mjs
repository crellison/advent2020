/**
 * @param {String} input 
 */
export const part1 = (input) => {
  let possibleGameIdSum = 0
  input.split('\n').forEach(line => {
    if (!line) return;
    const [substring, gameNum] = line.match(/^Game (\d+): /);
    const draws = line.match(/(\d\d+) (blue|red|green)/g);
    if (draws) {
      for (const cubeCount of draws) {
        const [count, color] = cubeCount.split(' ');
        if (color === 'red' && count > 12) return;
        if (color === 'green' && count > 13) return;
        if (color === 'blue' && count > 14) return;
      }
    }
    possibleGameIdSum += +gameNum
  })
  return possibleGameIdSum;
}

/**
 * @param {String} input 
 */
export const part2 = (input) => {
  let powerSum = 0
  input.split('\n').forEach(line => {
    if (!line) return;
    const minCubes = {
      red: 0, green: 0, blue: 0
    };
    const draws = line.match(/(\d+) (blue|red|green)/g);
    if (draws) {
      for (const cubeCount of draws) {
        const [count, color] = cubeCount.split(' ');
        if (minCubes[color] < +count) {
          minCubes[color] = +count;
        }
      }
    }
    powerSum += minCubes['red'] * minCubes['green'] * minCubes['blue'];
  })
  return powerSum;
}
