'use strict';

const fs = require('fs');
const path = require('path');

const async = require('async');
const {watch} = require('chokidar');
const {Environment, FileSystemLoader} = require('nunjucks');

const tmuxConfig = require('./config');


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

function getContext(version) {
    return {
        tmux_tmp_envs: tmuxConfig.tmpEnvs,
        version,
    };
}

function generateConfig(template, version, callback) {
    const context = getContext(version);
    renderConfig(template, context, (err, data) => {
        if (err) return callback(err);
        writeConfig(version, data, callback);
    })
}

function render(callback) {
    async.each(tmuxConfig.versions,
        async.apply(generateConfig, loadTemplate()),
        callback);
}

function removeBlankLines(data) {
    return data.replace(/^\s*[\r\n]/gm, '');
}

function renderConfig(template, context, callback) {
    template.render(context, (err, data) => {
        if (err) return callback(err);
        callback(null, removeBlankLines(data));
    });
}

class Tmux {
    renderAndWatch(callback) {
        const watcher = watch(tmuxConfig.template.path, {awaitWriteFinish: true});
        watcher.on('change', (event, path) => render(callback));
        render(callback);
    }
}

module.exports = {
    Tmux,
};
