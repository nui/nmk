const fs = require('fs');
const path = require('path');
const async = require('async');

const Compiler = require('../lib/Compiler');

class Zsh extends Compiler {
    concatFiles(files, callback) {
        async.map(files, fs.readFile, (err, arr) => {
            if (err) return callback(err);
            callback(null, arr.join(''));
        });
    }

    listZshrcSourceFiles(callback) {
        const zshrcSourceDir = this.options.zshrc.sourceDir;
        fs.readdir(zshrcSourceDir, (err, files) => {
            if (err) return callback(err);
            const getFilePath = async.asyncify(path.join.bind(null, zshrcSourceDir));
            async.map(files, getFilePath, callback);
        });
    }

    writeZshRc(data, callback) {
        const zshrc = path.join(this.options.zdotdir, '.zshrc');
        fs.writeFile(zshrc, data, callback);
    }

    renderZshrc(callback) {
        async.seq(
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
