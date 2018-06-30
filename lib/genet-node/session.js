const native = require('./binding')
const { Token } = native
const { Disposable } = require('disposables')
const { EventEmitter } = require('events')
const FilterCompiler = require('./filter')
function consume (len, layerStack, indexStack) {
  const indices = indexStack.splice(0, len)
  const layers = layerStack.splice(0, len)
  for (let index = 0; index < indices.length; index += 1) {
    layers[index].children = (layers[index].children || [])
      .concat(consume(indices[index], layerStack, indexStack))
  }
  return layers
}

function treefy (layerStack, indexStack) {
  const layers = [].concat(layerStack)
  const indices = [].concat(indexStack)
  const root = consume(1, layers, indices)
  const len = layerStack.length - layers.length
  if (indices.length >= len) {
    consume(len, [].concat(layerStack), indices)
  }
  return root
}

class Frame {
  constructor (frame) {
    this._frame = frame
    this._root = null
  }

  get index () {
    return this._frame.index
  }

  get root () {
    if (!this._root) {
      [this._root] = treefy(this._frame.layers, this._frame.treeIndices)
    }
    return this._root
  }

  get primary () {
    if (!this.root) {
      return null
    }
    function fistChild (layer) {
      if (layer.children.length === 0) {
        return layer
      }
      return fistChild(layer.children[0])
    }
    return fistChild(this.root)
  }

  query (id) {
    return this._frame.query(id)
  }
}

class Session extends EventEmitter {
  constructor (profile) {
    super()
    this._sess = new native.Session(profile)
    this._sess.callback = (event) => {
      switch (event.type) {
        case 'frames':
          this._status.frames = event.length
          break
        case 'filtered_frames':
          this._status.filters[Token.string(event.id)] = { frames: event.length }
          break
        default:
      }
      this.emit('update', event)
    }
    this._streams = []
    this._streamReaders = new Set()
    this._status = {
      filters: {},
      frames: 0,
    }
  }

  close () {
    this._sess.close()
  }

  frames (start, end) {
    return this._sess
      .frames(start, end)
      .map((frame) => new Frame(frame))
  }

  filteredFrames (id, start, end) {
    return this._sess
      .filteredFrames(Token.get(id), start, end)
      .map((frame) => new Frame(frame))
  }

  get status () {
    return this._status
  }

  setFilter (id, filter = '') {
    const filterCompiler = new FilterCompiler()
    const body = filterCompiler.compile(filter, { built: false }).linked
    this._sess.setFilter(Token.get(id), body)
    if (body === '') {
      Reflect.deleteProperty(this._status.filters, id)
    }
  }

  createReader (id, arg = '') {
    const handle = this._sess.createReader(id, arg)
    if (handle === 0) {
      throw new Error(`unregistered ID: ${id}`)
    }
    return new Disposable(() => {
      this._sess.closeReader(handle)
    })
  }

  createWriter (id, arg = '') {
    const handle = this._sess.createWriter(id, arg)
    if (handle === 0) {
      throw new Error(`unregistered ID: ${id}`)
    }
    return new Disposable(() => {
      this._sess.closeWriter(handle)
    })
  }

  regiterStreamReader (id, arg = '') {
    const reader = {
      id,
      arg,
    }
    this._streamReaders.add(reader)
    return new Disposable(() => {
      this._streamReaders.delete(reader)
    })
  }

  startStream () {
    this.stopStream()
    this._streams = Array.from(this._streamReaders)
      .map(({ id, arg }) => this.createReader(id, arg))
  }

  stopStream () {
    for (const handle of this._streams) {
      handle.dispose()
    }
    this._streams = []
  }

  get length () {
    return this._sess.length
  }
}

class Profile extends native.SessionProfile {}

Reflect.defineProperty(Session, 'Profile', { value: Profile })

module.exports = Session
