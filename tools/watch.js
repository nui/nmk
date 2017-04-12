const Tmux = require('../lib/Tmux');
const Zsh = require('../lib/Zsh');

const tmuxConfig = require('../tmux/tmux.config');
const zshConfig = require('../zsh/zsh.config');


function logSuccess(message) {
    return function (err) {
        if (err) throw err;
        console.log(message);
    };
}

module.exports = function () {
    new Tmux(tmuxConfig).watch(logSuccess('Rendered tmux configuration files.'));
    new Zsh(zshConfig).watch(logSuccess('Rendered .zshrc file'));
};
