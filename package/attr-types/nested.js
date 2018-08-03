const m = require('mithril')
const genet = require('@genet/api')
class Nested {
  view (vnode) {
    const keys = vnode.attrs.layer.attrs
      .filter((attr) => attr.id.startsWith(`${vnode.attrs.attr.id}.`))
      .map((attr) => {
        const { id } = attr
        return genet.session.tokenName(id)
      })
      .join(', ')
    return m('span', [keys])
  }
}
module.exports = Nested
