import {Tmux} from "../tmux/renderer";
import * as zsh from "../zsh/zsh";


function logSuccess(message: string) {
    return function (err) {
        if (err) throw err;
        console.log(message);
    };
}

export default function () {
    new Tmux().renderAndWatch(logSuccess('Rendered tmux configuration files.'));
    zsh.getRenderer().renderAndWatch(logSuccess('Rendered .zshrc file'));
}
