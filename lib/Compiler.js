const {watch} = require('chokidar');

function noop() {
}

class Compiler {
    constructor(options) {
        this.options = options;
    }

    run(callback) {
        throw new Error('not implemented in child class');
    }

    watch(callback) {
        if (typeof callback === 'undefined')
            callback = noop;
        const watcher = watch(this.options.watch.paths, {awaitWriteFinish: true});
        watcher.on('change', (event, path) => this.run(callback));
        this.run(callback);
    }
}

module.exports = Compiler;
