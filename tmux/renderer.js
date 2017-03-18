const fs = require('fs');
const path = require('path');
const async = require('async');

const {watch} = require('chokidar');
const {Environment, FileSystemLoader} = require('nunjucks');

const tmuxConfig = require('./config');

// private variables
let _template;
let _watcher;

function loadTemplate() {
    const opt = {trimBlocks: true}; // trimBlocks seems to behave not the same as Jinja2 version.
    const env = new Environment(new FileSystemLoader(tmuxConfig.dir), opt);
    return env.getTemplate(tmuxConfig.template.name, true);
}

function writeConfig(version, data, callback) {
    const versionStr = version.toFixed(1);
    const configFile = path.join(tmuxConfig.dir, `${versionStr}.conf`);
    fs.writeFile(configFile, data, callback);
}

function removeBlankLines(data) {
    return data.replace(/^\s*[\r\n]/gm, '');
}

function render(callback) {
    _template = loadTemplate();
    function iteratee(version, iterateeCallback) {
        renderConfig(version, (err, data) => {
            if (err) return iterateeCallback(err);
            writeConfig(version, removeBlankLines(data), iterateeCallback);
        });
    }

    async.each(tmuxConfig.versions, iteratee, callback);
}

function renderConfig(version, callback) {
    const context = {
        tmux_tmp_envs: tmuxConfig.tmpEnvs,
        version,
    };
    _template.render(context, callback);
}

class Tmux {
    renderAndWatch(callback) {
        _watcher = watch(tmuxConfig.template.path, {awaitWriteFinish: true});
        _watcher.on('change', (event, path) => render(callback));
        render(callback);
    }
}

module.exports = {
    Tmux,
};
