import fs from 'fs';
import path from 'path';

import async from 'async';
import {watch} from 'chokidar';
import {Environment, FileSystemLoader} from 'nunjucks';

import tmuxConfig from '../../tmux/config';


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

export default Tmux;
