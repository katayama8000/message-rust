const makeShortString = (content: string, maxContentLength: number) => {
  if (content.length > maxContentLength) {
    return content.substring(0, maxContentLength) + "...";
  }
  return content;
};

module.exports = makeShortString;
