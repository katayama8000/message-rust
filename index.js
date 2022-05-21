const makeShortString = (content, maxContentLength) => {
  if (content.length > maxContentLength) {
    return content.substring(0, maxContentLength) + "...";
  }
  return content;
};

module.exports = makeShortString;
