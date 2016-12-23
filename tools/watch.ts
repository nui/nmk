import {Tmux} from "../tmux/renderer";
import {Zsh} from "../zsh/renderer";


function logSuccess(message: string) {
    return function (err) {
        if (err) throw err;
        console.log(message);
    };
}

export default function () {
    new Tmux().renderAndWatch(logSuccess('Rendered tmux configuration files.'));
    new Zsh().renderAndWatch(logSuccess('Rendered .zshrc file'));
}
