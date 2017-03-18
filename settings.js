const path = require('path');

const NMK_DIR = __dirname;

module.exports = {
    pathogen: {
        src: 'https://raw.githubusercontent.com/tpope/vim-pathogen/master/autoload/pathogen.vim',
        target: path.join(NMK_DIR, 'vim/autoload/pathogen.vim'),
    }
};
