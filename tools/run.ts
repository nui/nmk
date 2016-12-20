function loadTool(name: string) {
    let path = `./${name}`;
    return require(path).default;
}

if (process.mainModule.children.length === 0 && process.argv.length > 2) {
    const args = process.argv.slice(3);
    loadTool(process.argv[2]).apply(null, args);
}
