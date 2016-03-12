import console from 'console';

import Promise from 'bluebird';
let fs = Promise.promisifyAll(require('fs'));
import request from 'request-promise';

import {pathogen} from '../settings';

let isFileNotFoundError = (err) => err instanceof Error && err.code === 'ENOENT';

function readLocalPathogen() {
    return fs.readFileAsync(pathogen.target, 'utf8').catch(reason => {
        return isFileNotFoundError(reason) ? null : Promise.reject(reason);
    });
}

function writePathogen(data) {
    return fs.writeFileAsync(pathogen.target, data);
}

function updateVimPathogen() {
    let local_pathogen = readLocalPathogen();
    let remote_pathogen = request(pathogen.src);
    return Promise.all([local_pathogen, remote_pathogen])
        .then(([local_data, data]) => {
            if (local_data !== data) {
                return writePathogen(data).then(() => {
                    console.log('Updated vim pathogen.');
                });
            }
            else {
                console.log('Pathogen is up-to-date.');
            }
        });
}

export default function() {
    updateVimPathogen();
};
