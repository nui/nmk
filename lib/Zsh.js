const fs = require('fs');
const path = require('path');
const asyncLib = require('async');

const Compiler = require('./Compiler');

class Zsh extends Compiler {
    concatFiles(files, callback) {
        asyncLib.map(files, fs.readFile, (err, arr) => {
            if (err) return callback(err);
            callback(null, arr.join(''));
        });
    }

    listZshrcSourceFiles(callback) {
        const zshrcSourceDir = this.options.zshrc.sourceDir;
        fs.readdir(zshrcSourceDir, (err, files) => {
            if (err) return callback(err);
            const getFilePath = asyncLib.asyncify(path.join.bind(null, zshrcSourceDir));
            asyncLib.map(files, getFilePath, callback);
        });
    }

    writeZshRc(data, callback) {
        const zshrc = path.join(this.options.zdotdir, '.zshrc');
        fs.writeFile(zshrc, data, callback);
    }

    renderZshrc(callback) {
        asyncLib.seq(
            this.listZshrcSourceFiles,
            this.concatFiles,
            this.writeZshRc
        ).call(this, callback);
    }

    run(callback) {
        this.renderZshrc(callback);
    }
}

module.exports = Zsh;
