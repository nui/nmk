function loadTool(name) {
    let path = `./${name}`;
    return require(path).default;
}

if (process.mainModule.children.length === 0 && process.argv.length > 2) {
    loadTool(process.argv[2])();
}
