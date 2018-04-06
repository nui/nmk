'use strict';

const path = require('path');

const tmuxDir = __dirname;
const templateName = 'tmux.conf.njk';
const templatePath = path.join(tmuxDir, templateName);

module.exports = {
    versions: [
        2.1,
        2.2,
        2.3,
        // 2.4, this one is buggy, avoid it
        2.5,
        2.6
    ],
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
