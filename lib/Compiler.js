'use strict';

const Tapable = require('tapable');
const {watch} = require('chokidar');

function throwError(err) {
    if (err)
        throw err;
}

class Watching {
    constructor(compiler, watchOptions, handler) {
        if (typeof handler === 'undefined')
            handler = throwError;
        const watcher = watch(watchOptions.paths, {awaitWriteFinish: true});
        watcher.on('change', (event, path) => compiler.run(handler));
        compiler.run(handler);
    }
}

class Compiler extends Tapable {
    constructor(options) {
        super();
        this.options = options;
    }

    watch(handler) {
        return new Watching(this, this.options.watch, handler);
    }
}

module.exports = Compiler;
