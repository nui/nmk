const fs = require('fs');
const path = require('path');

const async = require('async');
const {Environment, FileSystemLoader} = require('nunjucks');

const Compiler = require('../lib/Compiler');

function removeBlankLines(data) {
    return data.replace(/^\s*[\r\n]/gm, '');
}

class Tmux extends Compiler {
    writeConfig(version, data, callback) {
        const versionStr = version.toFixed(1);
        const configFile = path.join(this.options.dir, `${versionStr}.conf`);
        fs.writeFile(configFile, data, callback);
    }

    renderConfig(template, context, callback) {
        template.render(context, (err, data) => {
            if (err) return callback(err);
            callback(null, removeBlankLines(data));
        });
    }

    loadTemplate() {
        const opt = {trimBlocks: true}; // trimBlocks seems to behave not the same as Jinja2 version.
        const env = new Environment(new FileSystemLoader(this.options.dir), opt);
        return env.getTemplate(this.options.template.name, true);
    }

    getContext(version) {
        return {
            tmux_tmp_envs: this.options.tmpEnvs,
            version,
        };
    }

    generateConfig(template, version, callback) {
        const context = this.getContext(version);
        this.renderConfig(template, context, (err, data) => {
            if (err) return callback(err);
            this.writeConfig(version, data, callback);
        })
    }

    run(callback) {
        async.each(
            this.options.versions,
            async.apply(this.generateConfig.bind(this), this.loadTemplate()),
            callback);
    }
}

module.exports = Tmux;
