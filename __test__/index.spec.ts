import test from 'ava'

import { tachyon } from '../index'

test('list routes', (t) => {
  const server = tachyon()
  const routes = server.routes()
  t.deepEqual(routes, [])
})

test('create route', (t) => {
  const server = tachyon()
  server.get('/test', () => {})
  t.deepEqual(server.routes(), ['/test GET'])
})
