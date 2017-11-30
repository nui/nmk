'use strict';

const TmuxCompiler = require('../lib/TmuxCompiler');
const TmuxPlugin = require('./TmuxPlugin');

function tmux(options) {
    const compiler = new TmuxCompiler(options);
    new TmuxPlugin().apply(compiler);
    return compiler;
}

module.exports = tmux;
