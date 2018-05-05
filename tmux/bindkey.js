const V26 = 2.6;
const V27 = 2.7;

let f4 = (v) => v >= V27 ? "-sZ" : "-s";

exports.bindkey = (version) => ({
    f4: f4(version)
});

