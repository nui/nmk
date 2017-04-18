const path = require('path');

const zdotdir = __dirname;
const zshrcSourceDir = path.join(zdotdir, 'src', 'zshrc');

module.exports = {
    watch: {
        paths: path.join(zshrcSourceDir, '*.zsh'),
    },
    zdotdir,
    zshrc: {
        sourceDir: zshrcSourceDir,
    }
};
