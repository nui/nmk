const {Tmux} = require('../tmux/renderer');
const zsh = require('../zsh/zsh');


function logSuccess(message) {
    return function (err) {
        if (err) throw err;
        console.log(message);
    };
}

module.exports = function () {
    new Tmux().renderAndWatch(logSuccess('Rendered tmux configuration files.'));
    zsh.getRenderer().renderAndWatch(logSuccess('Rendered .zshrc file'));
};
