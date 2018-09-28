#!/usr/bin/env node

const { readJson } = require('fs-extra')
const path = require('path')

readJson(path.join(__dirname, '../package.json'), (err, obj) => {
  console.log(Object.keys({ ...obj.dependencies, ...obj.devDependencies }).join(','))
})
