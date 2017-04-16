'use strict';

const {Environment, FileSystemLoader} = require('nunjucks');


class TmuxTemplate {
    constructor(templateOptions) {
        const opt = {trimBlocks: true}; // trimBlocks seems to behave not the same as Jinja2 version.
        const env = new Environment(new FileSystemLoader(templateOptions.dir), opt);

        this.template = env.getTemplate(templateOptions.name, true);
    }

    render() {
        this.template.render(...arguments);
    }
}

module.exports = TmuxTemplate;
