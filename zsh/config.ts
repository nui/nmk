import * as path from "path";

const zdotdir = __dirname;

export default {
    zdotdir,
    zshrc: {
        dir: path.join(zdotdir, 'zshrc.src'),
        pattern: `${zdotdir}/zshrc.src/*.zsh`,
    }
};
