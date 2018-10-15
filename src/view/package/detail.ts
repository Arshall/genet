import ButtonBoxView from './button'
import ReadmeView from './readme'
import Env from '../../lib/env'
import SchemaInput from '../../lib/schema-input'
import titleCase from 'title-case'
import genet from '@genet/api'
import m from 'mithril'
import path from 'path'

let installerCallback: (any) => void = () => { }

export default class DetailView {
  private output: any
  constructor() {
    this.output = {}
  }

  view(vnode) {
    const { pkg } = vnode.attrs
    if (pkg === undefined) {
      return m('p', ['No package selected'])
    }

    const config = Object.entries(genet.config.schema)
      .filter(([id]) => id.startsWith(`${pkg.metadata.name}.`)) as [string, any][]
    return m('article', [
      m('h1', { disabled: pkg.disabled }, [
        pkg.metadata.name,
        m('span', { class: 'version' },
          [pkg.metadata.version]),
        m('span', {
          class: 'version',
          style: {
            display: pkg.abi
              ? 'inline'
              : 'none',
          },
        }, ['abi: ', pkg.abi]),
      ]),
      m('p', [pkg.metadata.description]),
      m('p', {
        style: {
          color: 'var(--theme-error)',
          display: pkg.incompatible
            ? 'block'
            : 'none',
        },
      }, [
          'This package is incompatible with the running genet version.',
          m('br'),
          `Required genet Version: ${pkg.metadata.engines.genet}`
        ]),
      m(ButtonBoxView, {
        pkg,
      }),
      m(ReadmeView, { dir: pkg.dir }),
      m('p', config.map(([id, schema]) => m('section', [
        m('h4', [
          schema.title || titleCase(id.split('.').slice(-1)[0]),
          m('span', { class: 'schema-path' }, [id])]),
        m(SchemaInput, {
          id,
          schema,
        }),
        m('p', { class: 'description' }, [schema.description])
      ]))),
      m('pre', { class: 'output' }, [
        this.output[pkg.id]
      ])
    ])
  }

  onupdate(vnode) {
    const { pkg } = vnode.attrs
    if (pkg !== null) {
      installerCallback = (chunk) => {
        this.output[pkg.id] = (this.output[pkg.id] || '') + chunk
        m.redraw()
      }
    }
  }
}
