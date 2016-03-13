/* @flow */
import Tmux from './renderers/tmux';
import Zsh from './renderers/zsh';


function onRenderSuccess(message: string) {
    return function (err) {
        if (err) throw err;
        console.log(message);
    };
}

export default function () {
    Tmux.renderAndWatch(onRenderSuccess('Rendered tmux configuration files.'));
    Zsh.renderAndWatch(onRenderSuccess('Rendered .zshrc file'));
}
