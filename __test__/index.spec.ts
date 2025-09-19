import test from 'ava'

import { tachyon } from '../index'

test('list routes', (t) => {
  const server = tachyon()
  const routes = server.routes()
  t.deepEqual(routes, [])
})

test('create route', (t) => {
  const server = tachyon()
  server.get('/test', (req, res) => {
    res.send('Hello World!')
  })
  t.deepEqual(server.routes(), ['/test GET'])
})

test('create POST route', (t) => {
  const server = tachyon()
  server.post('/users', (req, res) => {
    res.send('User created!')
  })
  t.deepEqual(server.routes(), ['/users POST'])
})

test('create PUT route', (t) => {
  const server = tachyon()
  server.put('/users/1', (req, res) => {
    res.send('User updated!')
  })
  t.deepEqual(server.routes(), ['/users/1 PUT'])
})

test('create DELETE route', (t) => {
  const server = tachyon()
  server.delete('/users/1', (req, res) => {
    res.send('User deleted!')
  })
  t.deepEqual(server.routes(), ['/users/1 DELETE'])
})

test('create PATCH route', (t) => {
  const server = tachyon()
  server.patch('/users/1', (req, res) => {
    res.send('User patched!')
  })
  t.deepEqual(server.routes(), ['/users/1 PATCH'])
})

test('create multiple routes', (t) => {
  const server = tachyon()
  server.get('/', (req, res) => {
    res.send('Home')
  })
  server.post('/users', (req, res) => {
    res.send('Create user')
  })
  server.put('/users/1', (req, res) => {
    res.send('Update user')
  })
  const routes = server.routes()
  t.is(routes.length, 3)
  t.true(routes.includes('/ GET'))
  t.true(routes.includes('/users POST'))
  t.true(routes.includes('/users/1 PUT'))
})
