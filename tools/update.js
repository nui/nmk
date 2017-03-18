const fs = require('fs');

const request = require('request-promise');
const {pathogen} = require('../settings');

let isFileNotFoundError = (err) => err.code === 'ENOENT';

function readLocalPathogen() {
    return new Promise((resolve, reject) => {
        fs.readFile(pathogen.target, 'utf8', (err, data) => {
            if (err)
                return isFileNotFoundError(err) ? resolve(null) : reject(err);
            resolve(data);
        })
    });
}

function writePathogen(data) {
    return new Promise((resolve, reject) => {
        fs.writeFile(pathogen.target, data, (err) => {
            if (err)
                return reject(err);
            resolve(data);
        });
    });
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

module.exports = function () {
    updatePathogen();
};
