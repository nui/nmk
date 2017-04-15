const tmux = require('../lib/tmux');
const Zsh = require('../lib/Zsh');

const zshConfig = require('../zsh/zsh.config');


function logSuccess(message) {
    return function (err) {
        if (err) throw err;
        console.log(message);
    };
}

module.exports = function () {
    new tmux(require('../tmux/tmux.config')).watch(logSuccess('Regenerated tmux configuration'));
    new Zsh(zshConfig).watch(logSuccess('Regenerated .zshrc'));
};
