import { tachyon } from '../index'

const app = tachyon()
app.get('/regular', (_req, res) => {
  res.send({ message: 'Hello World!' })
})
app.get('/json', async (_req, res) => {
  res.send({ message: 'Hello World!' })
})
app.post('/post', (req, res) => {
  const body = req.body
  res.send(body)
})
app.get('/empty', async (_req, res) => {
  res.status(204).send()
})
console.log(app.routes())
console.log('🚀 Tachyon server starting...')
app.listen(5000)
console.log('📡 Server listening on http://127.0.0.1:5000')
console.log('💡 Available routes:')
