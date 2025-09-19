import { tachyon } from '../index'

const app = tachyon()
app.get('/regular', (_req, res) => {
  res.send('Hello World!')
})
console.log('🚀 Tachyon server starting...')
app.listen(5000)
console.log('📡 Server listening on http://127.0.0.1:5000')
console.log('💡 Available routes:')
