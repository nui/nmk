const path = require('path');
const {Zsh} = require('./renderer');

const zdotdir = __dirname;

function getRenderer() {
    const settings = {
        zdotdir,
        zshrc: {
            sourceDir: path.join(zdotdir, 'zshrc.src'),
            sourcePattern: `${zdotdir}/zshrc.src/*.zsh`,
        }
    };
    return new Zsh(settings);
}

module.exports = {
    getRenderer,
};