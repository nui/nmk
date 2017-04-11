const path = require('path');

const zdotdir = __dirname;

module.exports = {
    watch: {
        paths: `${zdotdir}/zshrc.src/*.zsh`,
    },
    zdotdir,
    zshrc: {
        sourceDir: path.join(zdotdir, 'zshrc.src'),
    }
};
