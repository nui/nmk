import Promise from 'bluebird';
import request from 'request-promise';

var fs = Promise.promisifyAll(require('fs'));

import {pathogen} from '../settings';


function updateVimPathogen() {
    return request(pathogen.src)
        .then((data) => { 
            return fs.writeFileAsync(pathogen.target, data);
        })
        .then(() => console.log('Updated vim pathogen'));
}

export default function() {
    updateVimPathogen();
};
