import * as fs from "fs";
import * as Promise from "bluebird";
import * as request from "request-promise";
import {pathogen} from "../settings";
import {ENOENT} from "constants";
import ErrnoException = NodeJS.ErrnoException;


let isFileNotFoundError = (err: ErrnoException) => err.errno === ENOENT;

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

export default function () {
    updatePathogen();
};
