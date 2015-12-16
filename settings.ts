import * as path from 'path';

const NMK_DIR = __dirname;

export const pathogen = {
    src: 'https://raw.githubusercontent.com/tpope/vim-pathogen/master/autoload/pathogen.vim',
    target: path.join(NMK_DIR, 'vim/autoload/pathogen.vim'),
};

export default {
    pathogen
};
