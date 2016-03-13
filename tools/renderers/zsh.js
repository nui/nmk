/* @flow */
import fs from 'fs';
import path from 'path';

import async from 'async';
import {watch} from 'chokidar';

import zshConfig from '../../zsh/config';

declare function concatFilesCallback(err: any, data?: Buffer | string): void;

function concatFiles(files: Array<string>, callback: concatFilesCallback) {
    async.map(files, fs.readFile, (err, arr) => {
        if (err) return callback(err);
        callback(null, arr.join(''));
    });
}

function listZshrcSourceFiles(callback: Function) {
    const zshrcSourceDir = path.join(zshConfig.zdotdir, 'zshrc.src');
    fs.readdir(zshrcSourceDir, (err, files) => {
        if (err) return callback(err);
        const getFilePath = async.asyncify(path.join.bind(null, zshrcSourceDir));
        async.map(files, getFilePath, callback);
    });
}

function concatZshrc(callback: concatFilesCallback) {
    listZshrcSourceFiles((err, files) => {
        if (err) return callback(err);
        concatFiles(files, callback);
    });
}

function renderZshrc(callback: Function) {
    const zshrc = path.join(zshConfig.zdotdir, '.zshrc');
    concatZshrc((err, data) => {
        if (err) return callback(err);
        fs.writeFile(zshrc, data, callback);
    });
}

function render(callback) {
    renderZshrc(callback);
}

function renderAndWatch(callback: (err: any) => void) {
    const watcher = watch(zshConfig.zshrc.pattern, {awaitWriteFinish: true});
    watcher.on('change', (event, path) => render(callback));
    render(callback);
}

export default {
    renderAndWatch,
};