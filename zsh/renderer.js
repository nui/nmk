const fs = require('fs');
const path = require('path');
const async = require('async');

const {watch} = require('chokidar');


class Zsh {
    constructor(settings) {
        this.settings = settings;
    }

    concatFiles(files, callback) {
        async.map(files, fs.readFile, (err, arr) => {
            if (err) return callback(err);
            callback(null, arr.join(''));
        });
    }

    listZshrcSourceFiles(callback) {
        const zshrcSourceDir = this.settings.zshrc.sourceDir;
        fs.readdir(zshrcSourceDir, (err, files) => {
            if (err) return callback(err);
            const getFilePath = async.asyncify(path.join.bind(null, zshrcSourceDir));
            async.map(files, getFilePath, callback);
        });
    }

    concatZshrc(callback) {
        this.listZshrcSourceFiles((err, files) => {
            if (err) return callback(err);
            this.concatFiles(files, callback);
        });
    }

    renderZshrc(callback) {
        const zshrc = path.join(this.settings.zdotdir, '.zshrc');
        this.concatZshrc((err, data) => {
            if (err) return callback(err);
            fs.writeFile(zshrc, data, callback);
        });
    }

    render(callback) {
        this.renderZshrc(callback);
    }

    renderAndWatch(callback) {
        this.render(callback);
        this.watcher = watch(this.settings.zshrc.sourcePattern, {awaitWriteFinish: true});
        this.watcher.on('change', (event, path) => this.render(callback));
    }
}

module.exports = {
    Zsh,
};