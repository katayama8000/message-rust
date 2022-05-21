const makeShortString = (content, maxContentLength) => {
  if (content.length > maxContentLength) {
    return content.substring(0, maxContentLength) + "...";
  }
  return content;
};

console.log(makeShortString("Hello aaaaWorld", 10));

module.exports = makeShortString;
