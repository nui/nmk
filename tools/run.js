if (process.mainModule.children.length === 0 && process.argv.length > 2) {
    let module = require('./' + process.argv[2]).default;
    module();
}
