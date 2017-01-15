import watch from './watch';
import update from './update';

if (process.argv.length > 2) {
    const [tool, ...args] = process.argv.slice(2);
    let tools = { watch, update };
    tools[tool].apply(null, args);
}
