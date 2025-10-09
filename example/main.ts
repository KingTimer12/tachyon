import { tachyon } from '../index'

const app = tachyon()

// Exemplo de rota SYNC (síncrona)
app.get('/', (req, res) => {
  res.send({ message: 'Hello from sync handler!' })
})

// Exemplo de rota ASYNC (assíncrona)
app.get('/async', async (req, res) => {
  // Simula operação assíncrona
  await new Promise((resolve) => setTimeout(resolve, 10))
  res.send({ message: 'Hello from async handler!', async: true })
})

// POST sync
app.post('/sync-post', (req, res) => {
  res.send({ received: req.body, type: 'sync' })
})

// POST async
app.post('/async-post', async (req, res) => {
  // Simula processamento assíncrono
  await new Promise((resolve) => setTimeout(resolve, 5))
  res.send({ received: req.body, type: 'async', processed: true })
})

// PUT async
app.put('/users/:id', async (req, res) => {
  await new Promise((resolve) => setTimeout(resolve, 1))
  res.send({
    message: 'User updated',
    body: req.body,
    async: true,
  })
})

// DELETE sync
app.delete('/users/:id', (req, res) => {
  res.status(200).send({ message: 'User deleted', sync: true })
})

// PATCH async
app.patch('/items/:id', async (req, res) => {
  await new Promise((resolve) => setTimeout(resolve, 2))
  res.send({ message: 'Item patched', data: req.body })
})

// Rota com status code diferente (sync)
app.get('/not-found', (req, res) => {
  res.status(404).send({ error: 'Not found' })
})

// Rota com status code diferente (async)
app.get('/server-error', async (req, res) => {
  await new Promise((resolve) => setTimeout(resolve, 1))
  res.status(500).send({ error: 'Internal server error' })
})

// Rota que retorna vazio (sync)
app.get('/empty', (req, res) => {
  res.status(204).send()
})

// Rota que retorna vazio (async)
app.get('/empty-async', async (req, res) => {
  await new Promise((resolve) => setTimeout(resolve, 1))
  res.status(204).send()
})

console.log('🚀 Tachyon - The Fastest Node.js Framework')
console.log('='.repeat(50))
console.log('\n📋 Registered routes:')
console.log(app.routes())
console.log('\n⚡ Starting ultra-fast server...')
app.listen(5000)
console.log('✅ Server listening on http://127.0.0.1:5000')
console.log('\n💡 Test with:')
console.log('  curl http://localhost:5000/')
console.log('  curl http://localhost:5000/async')
console.log(
  '  curl -X POST http://localhost:5000/async-post -H "Content-Type: application/json" -d \'{"test": "data"}\'',
)
console.log('\n🎯 Both sync and async handlers are supported!')
