const V26 = 2.6;
const V27 = 2.7;

let chooseTree = (v) => v >= V27 ? "-sZ" : "-s";

exports.bindkey = (version) => ({
    chooseTree: chooseTree(version)
});
