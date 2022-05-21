"use strict";
const makeShortString = (content, maxContentLength) => {
    if (typeof content !== 'string') {
        throw new Error('content must be a string');
    }
    if (typeof maxContentLength !== 'number') {
        throw new Error('maxContentLength must be a number');
    }
    if (content.length > maxContentLength) {
        return content.substring(0, maxContentLength) + "...";
    }
    return content;
};
module.exports = makeShortString;
