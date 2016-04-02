import fs from 'mz/fs';
import Promise from 'bluebird';
import request from 'request-promise';
import {pathogen} from '../settings';

let isFileNotFoundError = (err) => err instanceof Error && err.code === 'ENOENT';

function readLocalPathogen() {
    return fs.readFile(pathogen.target, 'utf8').catch(reason => {
        return isFileNotFoundError(reason) ? null : Promise.reject(reason);
    });
}

function writePathogen(data) {
    return fs.writeFile(pathogen.target, data);
}

function updatePathogen() {
    return Promise.all([readLocalPathogen(), request(pathogen.src)])
        .then(([local, remote]) => {
            if (local !== remote) {
                return writePathogen(remote).then(() => {
                    console.log('Updated vim pathogen.');
                });
            }
            else {
                console.log('Pathogen is up-to-date.');
            }
        });
}

export default function() {
    updatePathogen();
};
