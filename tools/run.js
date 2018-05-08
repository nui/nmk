const update = require('./update');

if (process.argv.length > 2) {
    const [tool, ...args] = process.argv.slice(2);
    let tools = {update};
    tools[tool](...args)
}
