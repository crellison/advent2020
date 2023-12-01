import path from 'node:path';
import url from 'node:url';
import fs from 'node:fs';

const SRC_DIR = path.dirname(url.fileURLToPath(import.meta.url));
const NODE_2023 = path.resolve(SRC_DIR, './node_2023/')

const usage = "\
Usage\n\
$ node path/to/advent.mjs <day> <input-type>\n\
\n\
<day> num 1-25\n\
<input-suffix> string suffix for test input";

const parseArgs = (args) => {
  if (args.length > 4) throw new RangeError('Unexpected number of arguments');

  const day = parseInt(args[2]);
  if (isNaN(day)) throw new TypeError(`Unable to parse day as int: ${args[2]}`);

  const inputSuffix = args[3];
  return [day, inputSuffix]
}

const getDay = (day) => {
  const dayFile = fs.readdirSync(NODE_2023).find(value =>
    value.match(`day${day}_.*\.mjs`) !== null
  )

  if (dayFile === undefined) {
    throw new RangeError(`File for day ${day} not found in ${NODE_2023}`)
  }

  return path.resolve(NODE_2023, dayFile);
};

const getInput = (day, inputSuffix) => {
  const inputPath = path.resolve(SRC_DIR, `../input/2023/${day}-${inputSuffix}.txt`);
  if (!fs.existsSync(inputPath)) {
    throw new Error(`Requested input for ${day} with suffix <${inputSuffix}> does not exist at ${inputPath}`);
  }
  return fs.readFileSync(inputPath).toString();
}

const main = async () => {
  try {
    const [day, inputSuffix] = parseArgs(process.argv)
    const dayFile = getDay(day);
    const input = getInput(day, inputSuffix);

    const dayModule = await import(dayFile);

    console.log(`part one: ${dayModule.part1(input)}`)
    console.log(`part two: ${dayModule.part2(input)}`)
  } catch (err) {
    console.log(err.message);
    console.log(usage);
  }
}


main();
