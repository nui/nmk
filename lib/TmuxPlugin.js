'use strict';

function deleteEmptyLines(data) {
    return data.replace(/^\s*[\r\n]/gm, '');
}

class TmuxPlugin {
    apply(compiler) {
        compiler.hooks.afterRender.tap(this.constructor.name, (data) => {
            return deleteEmptyLines(data);
        });
    }
}

module.exports = TmuxPlugin;
