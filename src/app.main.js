import { app, BrowserWindow, ipcMain, webContents } from 'electron'
import PackageManager from './lib/package-manager'
import WindowFactory from './lib/window-factory'

if (require('electron-squirrel-startup')) {
  app.quit()
}

app.commandLine.appendSwitch('--enable-experimental-web-platform-features')
app.commandLine.appendSwitch('--ignore-gpu-blacklist')

async function init () {
  await PackageManager.init()
  await PackageManager.cleanup()
  WindowFactory.create(process.argv.slice(2))
}

app.on('ready', () => {
  init()
})

ipcMain.on('core:window:loaded', (event, id) => {
  const window = BrowserWindow.fromId(id)
  if (!window.isVisible()) {
    window.show()
  }
})

ipcMain.on('core:window:create', () => {
  WindowFactory.create(process.argv.slice(2))
})

const logContents = new Map()
ipcMain.on('core:logger:register', (event, windowId, contentId) => {
  logContents.set(windowId, contentId)
})
ipcMain.on('core:logger:message', (event, windowId, message) => {
  const contentId = logContents.get(windowId)
  if (Number.isInteger(contentId)) {
    const content = webContents.fromId(logContents.get(windowId))
    if (content !== null) {
      content.send('core:logger:message', message)
    }
  }
})
