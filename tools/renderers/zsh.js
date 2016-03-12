import fs from 'fs';
import path from 'path';

import async from 'async';
import {watch} from 'chokidar';

import zshConfig from '../../zsh/config';


function concatFiles(files, callback) {
    async.map(files, fs.readFile, (err, arr) => {
        if (err) return callback(err);
        callback(null, arr.join(''));
    });
}

class Zsh {
    static listZshrcSourceFiles(callback) {
        let zshrcSourceDir = path.join(zshConfig.zdotdir, 'zshrc.src');
        fs.readdir(zshrcSourceDir, (err, files) => {
            if (err) return callback(err);
            let getFilePath = async.asyncify(path.join.bind(null, zshrcSourceDir));
            async.map(files, getFilePath, callback);
        });
    }

    static concatZshrc(callback) {
        Zsh.listZshrcSourceFiles((err, files) => {
            if (err) return callback(err);
            concatFiles(files, callback);
        });
    }

    static renderZshrc(callback) {
        let zshrc = path.join(zshConfig.zdotdir, '.zshrc');
        Zsh.concatZshrc((err, data) => {
            if (err) return callback(err);
            fs.writeFile(zshrc, data, callback);
        });
    }

    static render(callback) {
        Zsh.renderZshrc(callback);
    }

    static renderAndWatch(callback) {
        let watcher = watch(zshConfig.zshrc.pattern, {awaitWriteFinish: true});
        watcher.on('change', (event, path) => Zsh.render(callback));
        Zsh.render(callback);
    }
}

export default Zsh;