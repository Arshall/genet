module.exports = function mac (attr) {
  return Array.from(attr.value).map((byte) =>
    (`0${byte.toString(16)}`).slice(-2))
    .join(':')
}
