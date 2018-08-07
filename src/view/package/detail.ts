import ButtonBoxView from './button'
import Env from '../../lib/env'
import Installer from '../../lib/package-install'
import SchemaInput from '../../lib/schema-input'
import titleCase from 'title-case'
import genet from '@genet/api'
import m from 'mithril'
import path from 'path'

let installerCallback: (any) => void = () => { }
async function install(pkg) {
  const shortName = pkg.id
  const installer = new Installer()
  installer.rustpath = genet.config.get('_.package.rustpath', '')
  installer.on('output', (chunk) => {
    installerCallback(chunk)
  })
  try {
    await installer.install(
      path.join(Env.userPackagePath, shortName), pkg.archive)
    genet.notify.show(
      `package: ${shortName}`, {
        type: 'success',
        title: 'Successfully installed',
      })
  } catch (err) {
    genet.notify.show(
      err.message, {
        type: 'error',
        title: 'Installation failed',
        ttl: 0,
      })
  }
  genet.packages.update()
}

export default class DetailView {
  private output: any
  constructor() {
    this.output = {}
  }

  view(vnode) {
    const { pkg } = vnode.attrs
    if (pkg === null) {
      return m('p', ['No package selected'])
    }

    const config = Object.entries(genet.config.schema)
      .filter(([id]) => id.startsWith(`${pkg.data.name}.`)) as [string, any][]
    return m('article', [
      m('h1', { disabled: pkg.disabled || pkg.incompatible }, [pkg.data.name,
      m('span', { class: 'version' },
        [pkg.data.version])]),
      m('p', [pkg.data.description]),
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
          `Required genet Version: ${pkg.data.engines.genet}`
        ]),
      m(ButtonBoxView, {
        pkg,
        install,
      }),
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
