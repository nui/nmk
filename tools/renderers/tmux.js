/* @flow */
import fs from 'fs';
import path from 'path';

import async from 'async';
import {watch} from 'chokidar';
import {Environment, FileSystemLoader} from 'nunjucks';

import tmuxConfig from '../../tmux/config';

declare function renderCallback(err: any): any;

// private variables
let _template;
let _watcher;

function loadTemplate () {
    const opt = {trimBlocks: true}; // trimBlocks seems to behave not the same as Jinja2 version.
    const env = new Environment(new FileSystemLoader(tmuxConfig.dir), opt);
    return env.getTemplate(tmuxConfig.template.name, true);
}

function writeConfig (version, data, callback) {
    const versionStr = version.toFixed(1);
    const configFile = path.join(tmuxConfig.dir, `${versionStr}.conf`);
    fs.writeFile(configFile, data, callback);
}

function render (callback: renderCallback) {
    _template = loadTemplate();
    function iteratee (version, iterateeCallback) {
        renderConfig(version, (err, data) => {
            if (err) return iterateeCallback(err);
            writeConfig(version, data, iterateeCallback);
        });
    }
    async.each(tmuxConfig.versions, iteratee, callback);
}

function renderConfig (version, callback) {
    const context = {
        tmux_tmp_envs: tmuxConfig.tmpEnvs,
        version,
    };
    _template.render(context, callback);
}

function renderAndWatch (callback?: renderCallback) {
    _watcher = watch(tmuxConfig.template.path, {awaitWriteFinish: true});
    _watcher.on('change', (event, path) => render(callback));
    render(callback);
}

export default {
    renderAndWatch,
};
