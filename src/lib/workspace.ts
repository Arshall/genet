import Config from './config'
import { Disposable } from './disposable'
import objpath from 'object-path'

const fields = Symbol('fields')
function flatten(object: object): any[] {
  if (typeof object !== 'object') {
    return object
  }
  let values: any[] = []
  for (const value of Object.values(object)) {
    values = values.concat(flatten(value))
  }
  return values
}

export default class Workspace extends Config {
  constructor(profile: string) {
    super(profile, 'workspace')
    this[fields] = { panels: new Map() }
  }

  registerPanel(id: string, panel) {
    const { panels } = this[fields]
    panels.set(id, panel)
    this.update()
    return new Disposable(() => {
      panels.delete(id)
    })
  }

  panel(id: string) {
    return this[fields].panels.get(id)
  }

  get panelLayout() {
    return this.get('_.panelLayout', {})
  }

  update() {
    const { panels } = this[fields]
    const layout = JSON.parse(JSON.stringify(this.get('_.panelLayout', {})))
    const activePanels = new Set(flatten(layout))
    for (const [id, panel] of panels) {
      if (!activePanels.has(id)) {
        objpath.insert(layout, `${panel.slot}.0`, id)
      }
    }
    this.set('_.panelLayout', layout)
  }
}
