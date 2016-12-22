import * as async from "async";
import * as fs from "fs";
import * as path from "path";
import {config} from "../../zsh/config";
import {watch} from "chokidar";
import {Renderer} from "./renderer";

export class Zsh implements Renderer {
    private watcher;

    static concatFiles(files: Array<string>, callback) {
        async.map(files, fs.readFile, (err, arr) => {
            if (err) return callback(err);
            callback(null, arr.join(''));
        });
    }

    static listZshrcSourceFiles(callback: Function) {
        const zshrcSourceDir = config.zshrc.sourceDir;
        fs.readdir(zshrcSourceDir, (err, files) => {
            if (err) return callback(err);
            const getFilePath = async.asyncify(path.join.bind(null, zshrcSourceDir));
            async.map(files, getFilePath, callback);
        });
    }

    static concatZshrc(callback) {
        Zsh.listZshrcSourceFiles((err, files) => {
            if (err) return callback(err);
            Zsh.concatFiles(files, callback);
        });
    }

    static renderZshrc(callback: Function) {
        const zshrc = path.join(config.zdotdir, '.zshrc');
        Zsh.concatZshrc((err, data) => {
            if (err) return callback(err);
            fs.writeFile(zshrc, data, callback);
        });
    }

    static render(callback) {
        Zsh.renderZshrc(callback);
    }

    renderAndWatch(callback?: Function) {
        Zsh.render(callback);
        this.watcher = watch(config.zshrc.sourcePattern, {awaitWriteFinish: true});
        this.watcher.on('change', (event, path) => Zsh.render(callback));
    }
}
