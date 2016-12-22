import * as path from "path";

const zdotdir = __dirname;

export const config = {
    zdotdir,
    zshrc: {
        sourceDir: path.join(zdotdir, 'zshrc.src'),
        sourcePattern: `${zdotdir}/zshrc.src/*.zsh`,
    }
};
