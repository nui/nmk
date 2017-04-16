'use strict';

function deleteEmptyLines(data) {
    return data.replace(/^\s*[\r\n]/gm, '');
}

class TmuxPlugin {
    apply(compiler) {
        compiler.plugin('after-render', (config) => {
            return deleteEmptyLines(config);
        });
    }
}

module.exports = TmuxPlugin;
