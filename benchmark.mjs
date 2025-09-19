import { tachyon } from './index.js'
import http from 'http'
import { performance } from 'perf_hooks'

const app = tachyon()

// Simple Hello World route
app.get('/', (req, res) => {
  res.send('Hello World!')
})

// JSON response route
app.get('/json', (req, res) => {
  res.json({ message: 'Hello JSON', timestamp: Date.now() })
})

// Start Tachyon server
console.log('Starting Tachyon server on port 3000...')
app.listen(3000)

// Wait a bit for server to start
await new Promise(resolve => setTimeout(resolve, 100))

// Benchmark function
async function benchmark(url, requests = 100) {
  console.log(`\nBenchmarking ${url} with ${requests} requests...`)

  const times = []
  let errors = 0

  for (let i = 0; i < requests; i++) {
    const start = performance.now()

    try {
      const response = await fetch(url)
      const text = await response.text()
      const end = performance.now()
      times.push(end - start)
    } catch (error) {
      errors++
    }
  }

  if (times.length === 0) {
    console.log('❌ All requests failed')
    return
  }

  // Calculate statistics
  times.sort((a, b) => a - b)
  const min = Math.min(...times)
  const max = Math.max(...times)
  const avg = times.reduce((a, b) => a + b, 0) / times.length
  const p50 = times[Math.floor(times.length * 0.5)]
  const p95 = times[Math.floor(times.length * 0.95)]
  const p99 = times[Math.floor(times.length * 0.99)]

  console.log('📊 Results:')
  console.log(`   Requests: ${times.length} successful, ${errors} failed`)
  console.log(`   Min:      ${min.toFixed(2)}ms`)
  console.log(`   Average:  ${avg.toFixed(2)}ms`)
  console.log(`   P50:      ${p50.toFixed(2)}ms`)
  console.log(`   P95:      ${p95.toFixed(2)}ms`)
  console.log(`   P99:      ${p99.toFixed(2)}ms`)
  console.log(`   Max:      ${max.toFixed(2)}ms`)

  return { min, avg, p50, p95, p99, max, errors }
}

// Run benchmarks
console.log('🚀 Tachyon Performance Benchmark')
console.log('================================')

await benchmark('http://localhost:3000/', 100)
await benchmark('http://localhost:3000/json', 100)

// Quick burst test
console.log('\n⚡ Burst Test (1000 requests)')
await benchmark('http://localhost:3000/', 1000)

console.log('\n✅ Benchmark completed!')
process.exit(0)
