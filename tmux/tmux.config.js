'use strict';

const path = require('path');

const tmuxDir = __dirname;
const templateName = 'tmux.conf.njk';
const templatePath = path.join(tmuxDir, templateName);

module.exports = {
    versions: [1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4],
    output: {
        dir: tmuxDir,
    },
    template: {
        dir: tmuxDir,
        name: templateName,
    },
    tmpEnvs: [
        'NMK_TMUX_256_COLOR',
        'NMK_TMUX_DEFAULT_SHELL',
        'NMK_TMUX_DEFAULT_TERMINAL',
        'NMK_TMUX_DETACH_ON_DESTROY',
        'NMK_TMUX_HISTORY',
    ],
    watch: {
        paths: templatePath,
    },
};
