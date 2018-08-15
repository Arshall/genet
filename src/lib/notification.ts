import { Disposable } from './disposable'
import m from 'mithril'
import marked from 'marked'

class Markdown {
  view() {
    return m('p')
  }

  oncreate(vnode) {
    vnode.dom.innerHTML = marked(vnode.attrs.content)
  }
}

class Container {
  view(vnode) {
    const { opt, content, handler } = vnode.attrs
    return [
      m('h4', {
        style: {
          display: opt.closeButton || opt.title
            ? 'block'
            : 'none',
        },
      }, [
          opt.title,
          m('button', {
            style: { visible: opt.closeButton },
            onclick: () => {
              handler.dispose()
            },
          }, ['Close'])
        ]),
      m(Markdown, { content })
    ]
  }
}

export default class Notification {
  private _container: HTMLElement | null
  constructor() {
    this._container = null
  }

  show(content: string, options = {}) {
    const opt = Object.assign({
      type: '',
      title: '',
      ttl: 5000,
      closeButton: true,
    }, options)
    if (this._container === null) {
      this._container = document.querySelector('div.notification')
    }
    const base = document.createElement('div')
    base.className = opt.type

    const handler = new Disposable(() => {
      base.remove()
    })
    m.mount(base, {
      view: () => m(Container, {
        opt,
        content,
        handler,
      }),
    })
    if (this._container !== null) {
      this._container.appendChild(base)
    }

    if (opt.ttl > 0) {
      setTimeout(() => {
        handler.dispose()
      }, opt.ttl)
    }
    return handler
  }
}
