'use strict';

function removeBlankLines(data) {
    return data.replace(/^\s*[\r\n]/gm, '');
}

class TmuxPlugin {
    apply(compiler) {
        compiler.plugin('after-render', (config) => {
            return removeBlankLines(config);
        });
    }
}

module.exports = TmuxPlugin;
