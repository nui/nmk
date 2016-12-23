import * as path from "path";
import {Settings} from "./interfaces";
import {Renderer} from "../ts/renderer";
import {Zsh} from "./renderer";

const zdotdir = __dirname;

export function getRenderer(): Renderer {
    const settings: Settings = {
        zdotdir,
        zshrc: {
            sourceDir: path.join(zdotdir, 'zshrc.src'),
            sourcePattern: `${zdotdir}/zshrc.src/*.zsh`,
        }
    };
    return new Zsh(settings);
}
