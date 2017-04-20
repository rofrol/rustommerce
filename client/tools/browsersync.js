#!/usr/bin/env node

var path = require('path');
var root = path.resolve(__dirname + '/../..');
require('dotenv').config({ path: root + '/.env' })
var browserSync = require('browser-sync');

var dist = path.resolve(root + '/client/dist')
browserSync({
  proxy: 'localhost:' + process.env.static_port,
  serveStatic: [],
  files: [dist],
  open: false,
  port: process.env.browsersync_port,
  reloadDebounce: 1000
});
