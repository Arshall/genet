import m from 'mithril'

function parseRange (exp) {
  const ranges = exp
    .trim()
    .split(',')
    .map((str) => str.replace(/\s+/g, '').trim()
      .split('-'))
    .map((nums) => {
      if (nums.length === 1 && nums[0] === '') {
        return null
      }
      const begin = Number.parseInt(nums[0] || 0, 10)
      const end = Number.parseInt(nums[1] || Number.MAX_SAFE_INTEGER, 10)
      if (nums.length === 1) {
        return [begin, begin]
      } else if (nums.length === 2) {
        if (begin < end) {
          return [begin, end]
        }
        return [end, begin]

      }
      return null
    })
    .filter((range) => range !== null)
  ranges.sort((lhs, rhs) => lhs[0] - rhs[0])
  const merged = []
  for (const range of ranges) {
    const last = merged[merged.length - 1]
    if (last && last[1] >= range[0]) {
      last[1] = Math.max(last[1], range[1])
    } else {
      merged.push(range)
    }
  }
  return merged.map((range) => {
    if (range[0] === 0 && range[1] === Number.MAX_SAFE_INTEGER) {
      return ''
    }
    if (range[0] === 0) {
      return `(index <= ${range[1]})`
    }
    if (range[1] === Number.MAX_SAFE_INTEGER) {
      return `(${range[0]} <= index)`
    }
    return `(${range[0]} <= $.index && $.index <= ${range[1]})`
  }).join(' || ')
}

export default class ExportDialog {
  constructor () {
    this.mode = genet.workspace.get('_.pcap.exporter.mode', 'all')
  }

  oncreate (vnode) {
    vnode.dom.querySelector(
      `input[type=radio][value=${this.mode}]`).checked = true
    vnode.dom.querySelector(
      'input[type=text][name=range]').value =
        genet.workspace.get('_.pcap.exporter.range', '')
    vnode.dom.querySelector(
      'input[type=text][name=filter]').value =
        genet.workspace.get('_.pcap.exporter.filter', '')
  }

  update (vnode) {
    this.mode = vnode.dom.querySelector(
      'input[type=radio]:checked').value

    process.nextTick(() => {
      genet.workspace.set('_.pcap.exporter.mode', this.mode)
      genet.workspace.set('_.pcap.exporter.range', vnode.dom.querySelector(
        'input[type=text][name=range]').value)
      genet.workspace.set('_.pcap.exporter.filter', vnode.dom.querySelector(
        'input[type=text][name=filter]').value)
    })
  }

  view (vnode) {
    return m('div', [
      m('ul', [
        m('li', [
          m('label', [
            m('input', {
              type: 'radio',
              name: 'filter',
              value: 'all',
              onchange: () => this.update(vnode),
            }),
            ' All Frames'
          ])
        ]),
        m('li', [
          m('label', [
            m('input', {
              type: 'radio',
              name: 'filter',
              value: 'visible',
              onchange: () => this.update(vnode),
            }),
            ' Visible Frames'
          ])
        ]),
        m('li', [
          m('label', [
            m('input', {
              type: 'radio',
              name: 'filter',
              value: 'checked',
              onchange: () => this.update(vnode),
            }),
            ` Checked Frames (${vnode.attrs.checkedFrames.size})`
          ])
        ]),
        m('li', [
          m('label', [
            m('input', {
              type: 'radio',
              name: 'filter',
              value: 'range',
              onchange: () => this.update(vnode),
            }),
            ' Index Range (starts from 1)'
          ])
        ]),
        m('li', [
          m('input', {
            type: 'text',
            name: 'range',
            placeholder: 'e.g. 1-20, 51, 60-',
            disabled: this.mode !== 'range',
            onchange: () => this.update(vnode),
          })
        ]),
        m('li', [
          m('label', [
            m('input', {
              type: 'radio',
              name: 'filter',
              value: 'filter',
              onchange: () => this.update(vnode),
            }),
            ' Custom Filter'
          ])
        ]),
        m('li', [
          m('input', {
            type: 'text',
            name: 'filter',
            placeholder: 'e.g. tcp.flags.ack',
            disabled: this.mode !== 'filter',
            onchange: () => this.update(vnode),
          })
        ]),
        m('li', [
          m('input', {
            type: 'button',
            value: 'Export',
            onclick: () => {
              let filter = ''
              switch (this.mode) {
                case 'visible':
                  filter = vnode.attrs.displayFilter
                  break
                case 'range':
                  filter = parseRange(vnode.dom.querySelector(
                    'input[type=text][name=range]').value)
                  break
                case 'checked':
                  {
                    const list = Array.from(
                      vnode.attrs.checkedFrames.values()).join(',')
                    filter = parseRange(list)
                  }
                  break
                case 'filter':
                  filter = vnode.dom.querySelector(
                    'input[type=text][name=filter]').value
                  break
                default:
              }
              vnode.attrs.callback(filter)
            },
          })
        ])
      ])
    ])
  }
}
