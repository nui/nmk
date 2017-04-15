'use strict';

const TmuxCompiler = require('../lib/TmuxCompiler');
const TmuxPlugin = require('./TmuxPlugin');

function tmux(options) {
    const compiler = new TmuxCompiler(options);
    compiler.apply(new TmuxPlugin());
    return compiler;
}

module.exports = tmux;
