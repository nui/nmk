import assert from 'assert';
import console from 'console';
import fs from 'fs';
import path from 'path';

import async from 'async';
import {watch} from 'chokidar';
import {Environment, FileSystemLoader} from 'nunjucks';

import tmuxConfig from '../tmux/config';
import zshConfig from '../zsh/config';


function concatFiles(files, callback) {
    async.map(files, fs.readFile, (err, arr) => {
        if (err) return callback(err);
        callback(null, arr.join(''));
    });
}

class Tmux {
    constructor() {
        this.template = null;
        this.watcher = null;

        this.render = this.render.bind(this);
        this.renderAndWatch = this.renderAndWatch.bind(this);
        this.renderConfig = this.renderConfig.bind(this);
    }

    static loadTemplate() {
        let opt = {trimBlocks: true}; // trimBlocks seems to not work.
        let env = new Environment(new FileSystemLoader(tmuxConfig.dir), opt);
        return env.getTemplate(tmuxConfig.template.name, true);
    }

    static writeConfig(version, data, callback) {
        let versionStr = version.toFixed(1);
        let configFile = path.join(tmuxConfig.dir, `${versionStr}.conf`);
        fs.writeFile(configFile, data, callback);
    }

    render(callback) {
        this.template = Tmux.loadTemplate();
        async.each(tmuxConfig.versions,
            (version, iterCb) => {
                this.renderConfig(version, (err, data) => {
                    if (err) return iterCb(err);
                    Tmux.writeConfig(version, data, iterCb);
                });
            },
            callback);
    }

    renderAndWatch(callback) {
        this.watcher = watch(tmuxConfig.template.path, {awaitWriteFinish: true});
        this.watcher.on('change', (event, path) => this.render(callback));
        this.render(callback);
    }

    renderConfig(version, callback) {
        let context = {
            tmux_tmp_envs: tmuxConfig.tmpEnvs,
            version
        };
        this.template.render(context, callback);
    }
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

export default function () {
    let tmux = new Tmux();
    tmux.renderAndWatch((err) => {
        if (err) throw err;
        console.log('Rendered tmux configuration files.');
    });
    Zsh.renderAndWatch((err) => {
        if (err) throw err;
        console.log('Rendered .zshrc file');
    });
}
