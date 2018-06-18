const m = require('mithril')
class BinaryItem {
  constructor () {
    this.range = [-1, -1]
  }
  oncreate () {
    genet.action.on('core:frame:range-selected', (range) => {
      this.range = range.length === 2
        ? range
        : [-1, -1]
      m.redraw()
    })
  }

  view (vnode) {
    const showHex = true
    const showAscii = true
    const { payload } = vnode.attrs
    return m('div', { class: 'binary-view' }, [
      m('ul', {
        class: 'hex-list',
        style: {
          display: showHex
            ? 'block'
            : 'none',
        },
      }, [
        (new Array(Math.ceil(payload.length / 16))).fill()
          .map((idx, line) => {
            const slice = payload.slice(line * 16, (line + 1) * 16)
            return m('li', [
              (new Array(slice.length)).fill()
                .map((item, byte) => {
                  const index = (line * 16) + byte
                  return m('span',
                    {
                      'data-selected':
                  this.range[0] <= index && index < this.range[1],
                    },
                    [(`0${payload[index].toString(16)}`).slice(-2)])
                })
            ])
          })
      ]),
      m('ul', {
        class: 'ascii-list',
        style: {
          display: showAscii
            ? 'block'
            : 'none',
        },
      }, [
        (new Array(Math.ceil(payload.length / 16))).fill()
          .map((str, line) => {
            const slice = payload.slice(line * 16, (line + 1) * 16)
            return m('li', [
              (new Array(slice.length)).fill()
                .map((item, byte) => {
                  const index = (line * 16) + byte
                  const char = payload[index]
                  const ascii = (char >= 0x21 && char <= 0x7e)
                    ? String.fromCharCode(char)
                    : '.'
                  return m('span',
                    {
                      'data-selected':
                      this.range[0] <= index && index < this.range[1],
                    },
                    [ascii])
                })
            ])
          })
      ])
    ])
  }
}

class BinaryView {
  constructor () {
    this.selectedFrame = null
    genet.action.on('core:frame:selected', (frame) => {
      this.selectedFrame = frame
      m.redraw()
    })
  }

  view () {
    if (this.selectedFrame === null) {
      return m('div', { class: 'binary-view' }, ['No frame selected'])
    }
    const frame = this.selectedFrame
    return m(BinaryItem, { payload: frame.rootLayer.payloads[0].slices[0] })
  }
}

module.exports = BinaryView
