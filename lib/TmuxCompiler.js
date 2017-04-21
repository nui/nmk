'use strict';

const fs = require('fs');
const path = require('path');

const asyncLib = require('async');

const Compiler = require('./Compiler');
const TmuxTemplate = require('./TmuxTemplate');


class TmuxCompiler extends Compiler {
    constructor(options) {
        super(options);
        this.outputOptions = options.output;

        this.template = null;
        this.versions = this.options.versions;
    }

    writeConfig(version, data, callback) {
        const versionStr = version.toFixed(1);
        const configFile = path.join(this.outputOptions.dir, `${versionStr}.conf`);
        fs.writeFile(configFile, data, callback);
    }

    renderConfig(context, callback) {
        this.template.render(context, (err, data) => {
            if (err) return callback(err);
            data = this.applyPluginsWaterfall0('after-render', data);
            callback(null, data);
        });
    }

    getContext(version) {
        return {
            tmpEnvs: this.options.tmpEnvs,
            version,
        };
    }

    generateConfig(version, callback) {
        const context = this.getContext(version);
        this.renderConfig(context, (err, data) => {
            if (err) return callback(err);
            this.writeConfig(version, data, callback);
        })
    }

    reloadTemplate() {
        this.template = new TmuxTemplate(this.options.template);
    }

    run(callback) {
        this.reloadTemplate();
        asyncLib.each(this.versions, this.generateConfig.bind(this), callback);
    }
}

module.exports = TmuxCompiler;
