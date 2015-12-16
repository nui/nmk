import * as async from "async";
import * as fs from "fs";
import * as path from "path";
import {watch} from "chokidar";

import {Renderer} from "../ts/interfaces";
import {Settings} from "./interfaces";


export class Zsh implements Renderer {
    private watcher;
    private settings: Settings;

    constructor(settings: Settings) {
        this.settings = settings;
    }

    private concatFiles(files: Array<string>, callback) {
        async.map(files, fs.readFile, (err, arr) => {
            if (err) return callback(err);
            callback(null, arr.join(''));
        });
    }

    private listZshrcSourceFiles(callback: Function) {
        const zshrcSourceDir = this.settings.zshrc.sourceDir;
        fs.readdir(zshrcSourceDir, (err, files) => {
            if (err) return callback(err);
            const getFilePath = async.asyncify(path.join.bind(null, zshrcSourceDir));
            async.map(files, getFilePath, callback);
        });
    }

    private concatZshrc(callback) {
        this.listZshrcSourceFiles((err, files) => {
            if (err) return callback(err);
            this.concatFiles(files, callback);
        });
    }

    private renderZshrc(callback: Function) {
        const zshrc = path.join(this.settings.zdotdir, '.zshrc');
        this.concatZshrc((err, data) => {
            if (err) return callback(err);
            fs.writeFile(zshrc, data, callback);
        });
    }

    private render(callback) {
        this.renderZshrc(callback);
    }

    renderAndWatch(callback?: Function) {
        this.render(callback);
        this.watcher = watch(this.settings.zshrc.sourcePattern, {awaitWriteFinish: true});
        this.watcher.on('change', (event, path) => this.render(callback));
    }
}
