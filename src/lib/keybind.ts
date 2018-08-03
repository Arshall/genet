import { Disposable } from './disposable'
import { EventEmitter } from 'events'
import Logger from './logger'
import Mousetrap from 'mousetrap'
import deepEqual from 'deep-equal'
import Env from './env'
import fs from 'fs-extra'
import genet from '@genet/api'
import path from 'path'
import yaml from 'js-yaml'

const fields = Symbol('fields')
function transformBindSet(map, binds) {
  for (const [selector, bind] of Object.entries(binds)) {
    for (const [key, action] of Object.entries(bind)) {
      map[key] = map[key] || []
      map[key].push({
        selector,
        action,
      })
    }
  }
}

export default class KeyBind extends EventEmitter {
  constructor(profile: string, logger: Logger) {
    super()
    const filePath =
      path.join(Env.userProfilePath, profile, 'keybind.yml')
    fs.ensureFileSync(filePath)

    this[fields] = {
      filePath,
      map: {},
      bindSets: new Set(),
      load: () => {
        let bind = null
        try {
          bind = yaml.safeLoad(fs.readFileSync(filePath, 'utf8'))
        } catch (err) {
          logger.warn(err)
        }
        this[fields].userBindSet = bind || {}
        this.update()
      },
    }

    this[fields].load()
    fs.watchFile(filePath, () => this[fields].load())
  }

  register(binds) {
    const { bindSets } = this[fields]
    bindSets.add(binds)
    this.update()
    return new Disposable(() => {
      bindSets.delete(binds)
      this.update()
    })
  }

  update() {
    const map = {}
    for (const binds of this[fields].bindSets) {
      transformBindSet(map, binds)
    }
    transformBindSet(map, this[fields].userBindSet)
    const keys = new Set()
    for (const key of Object.keys(map)) {
      keys.add(key)
    }
    for (const key of Object.keys(this[fields].map)) {
      keys.add(key)
    }
    for (const key of keys) {
      if (!deepEqual(map[key], this[fields].map[key])) {
        Mousetrap.unbind(key)
        if (key in map) {
          Mousetrap.bind(key, (event, combo) => {
            for (const binds of this[fields].map[combo]) {
              if (event.target.matches(binds.selector)) {
                genet.action.global.emit(binds.action)
                event.preventDefault()
                event.stopPropagation()
                break
              }
            }
          })
        }
      }
    }
    this[fields].map = map
    this.emit('update')
  }

  get keymap() {
    return this[fields].map
  }
}
