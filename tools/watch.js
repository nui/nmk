// @flow
import Tmux from './renderers/tmux';
import Zsh from './renderers/zsh';


function logSuccess(message: string) {
    return function (err) {
        if (err) throw err;
        console.log(message);
    };
}

export default function () {
    Tmux.renderAndWatch(logSuccess('Rendered tmux configuration files.'));
    Zsh.renderAndWatch(logSuccess('Rendered .zshrc file'));
}
