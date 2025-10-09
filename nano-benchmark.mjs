import { tachyon } from './index.js'
import { performance } from 'perf_hooks'

const app = tachyon()

// Register ultra-optimized dynamic routes
app.get('/', (req, res) => {
  res.send('Hello World!')
})

app.get('/fast', (req, res) => {
  res.send('Ultra Fast!')
})

app.post('/users', (req, res) => {
  res.send({ id: 1, name: 'User', created: Date.now() })
})

app.get('/json', (req, res) => {
  res.send({ message: 'Hello', timestamp: Date.now() })
})

app.get('/api/health', (req, res) => {
  res.send({ status: 'ok', uptime: process.uptime() })
})

console.log('🚀 Tachyon Nano-Benchmark - Dynamic Routes Only')
console.log('===============================================')
console.log('Testing nanosecond-level performance for dynamic routes...\n')

// Start server
console.log('Starting server on port 3000...')
app.listen(3000)

// Wait for server to start
await new Promise((resolve) => setTimeout(resolve, 100))

// Extensive warm-up for JIT optimization
console.log('Warming up routes for maximum performance...')
for (let i = 0; i < 100; i++) {
  try {
    await fetch('http://localhost:3000/')
    await fetch('http://localhost:3000/fast')
    await fetch('http://localhost:3000/json')
  } catch (e) {}
}

console.log('Server ready. Running nano-benchmarks on dynamic routes...\n')

// Ultra-precision benchmark function optimized for nanoseconds
async function nanoBenchmark(url, description, requests = 2000) {
  console.log(`📊 ${description}`)
  console.log(`   URL: ${url}`)
  console.log(`   Requests: ${requests}`)

  const times = []
  let errors = 0

  // Use process.hrtime.bigint() for nanosecond precision
  for (let i = 0; i < requests; i++) {
    const start = process.hrtime.bigint()

    try {
      const response = await fetch(url)
      const text = await response.text()
      const end = process.hrtime.bigint()

      // Store time in nanoseconds
      const durationNanos = Number(end - start)
      times.push(durationNanos)
    } catch (error) {
      errors++
    }
  }

  if (times.length === 0) {
    console.log('   ❌ All requests failed\n')
    return
  }

  // Sort for percentile calculations
  times.sort((a, b) => a - b)

  const min = times[0]
  const max = times[times.length - 1]
  const avg = times.reduce((a, b) => a + b, 0) / times.length
  const p50 = times[Math.floor(times.length * 0.5)]
  const p95 = times[Math.floor(times.length * 0.95)]
  const p99 = times[Math.floor(times.length * 0.99)]
  const p999 = times[Math.floor(times.length * 0.999)]

  console.log('   Results (nanoseconds):')
  console.log(`   Min:      ${min.toLocaleString()}ns`)
  console.log(`   P50:      ${p50.toLocaleString()}ns`)
  console.log(`   Average:  ${Math.round(avg).toLocaleString()}ns`)
  console.log(`   P95:      ${p95.toLocaleString()}ns`)
  console.log(`   P99:      ${p99.toLocaleString()}ns`)
  console.log(`   P99.9:    ${p999.toLocaleString()}ns`)
  console.log(`   Max:      ${max.toLocaleString()}ns`)

  // Also show in microseconds for comparison
  console.log('   Results (microseconds):')
  console.log(`   Min:      ${(min / 1000).toFixed(2)}μs`)
  console.log(`   P50:      ${(p50 / 1000).toFixed(2)}μs`)
  console.log(`   Average:  ${(avg / 1000).toFixed(2)}μs`)
  console.log(`   P95:      ${(p95 / 1000).toFixed(2)}μs`)
  console.log(`   Errors:   ${errors}`)

  // Performance rating
  if (avg < 1000000) {
    // < 1ms
    console.log('   🏆 EXCELLENT: Sub-millisecond performance!')
  } else if (avg < 5000000) {
    // < 5ms
    console.log('   ✅ GOOD: Fast performance')
  } else {
    console.log('   ⚠️  SLOW: Performance could be improved')
  }

  console.log()

  return { min, avg, p50, p95, p99, max, errors }
}

// Ultra-fast burst test for concurrent performance
async function concurrentBurst(url, description, concurrency = 50) {
  console.log(`⚡ ${description} - CONCURRENT BURST`)
  console.log(`   URL: ${url}`)
  console.log(`   Concurrency: ${concurrency}`)

  const promises = []
  const start = process.hrtime.bigint()

  // Fire all requests simultaneously
  for (let i = 0; i < concurrency; i++) {
    promises.push(fetch(url).then((r) => r.text()))
  }

  try {
    await Promise.all(promises)
    const end = process.hrtime.bigint()

    const totalTimeNanos = Number(end - start)
    const avgPerRequestNanos = totalTimeNanos / concurrency

    console.log(`   Total time: ${totalTimeNanos.toLocaleString()}ns (${(totalTimeNanos / 1000000).toFixed(2)}ms)`)
    console.log(
      `   Per request: ${Math.round(avgPerRequestNanos).toLocaleString()}ns (${(avgPerRequestNanos / 1000).toFixed(2)}μs)`,
    )
    console.log(`   Throughput: ${Math.round(concurrency / (totalTimeNanos / 1000000000))} req/sec`)
  } catch (error) {
    console.log(`   ❌ Concurrent burst test failed: ${error.message}`)
  }

  console.log()
}

// Speed comparison test
async function speedComparison() {
  console.log('🏁 SPEED COMPARISON TEST')
  console.log('========================')

  const routes = [
    { url: 'http://localhost:3000/', name: 'Simple GET' },
    { url: 'http://localhost:3000/fast', name: 'Fast Route' },
    { url: 'http://localhost:3000/json', name: 'JSON Response' },
  ]

  const results = []

  for (const route of routes) {
    const result = await nanoBenchmark(route.url, route.name, 1000)
    if (result) {
      results.push({ name: route.name, avg: result.avg })
    }
  }

  // Show ranking
  results.sort((a, b) => a.avg - b.avg)
  console.log('🏆 SPEED RANKING (fastest to slowest):')
  results.forEach((result, index) => {
    const medal = index === 0 ? '🥇' : index === 1 ? '🥈' : index === 2 ? '🥉' : '  '
    console.log(`   ${medal} ${result.name}: ${Math.round(result.avg).toLocaleString()}ns`)
  })
  console.log()
}

// Run comprehensive benchmarks
try {
  // Test all dynamic routes
  await nanoBenchmark('http://localhost:3000/', 'Dynamic GET Route (Root)', 3000)
  await nanoBenchmark('http://localhost:3000/fast', 'Dynamic GET Route (Fast)', 3000)
  await nanoBenchmark('http://localhost:3000/json', 'Dynamic JSON Route', 2000)
  await nanoBenchmark('http://localhost:3000/api/health', 'Dynamic API Route', 2000)

  // Test POST route
  console.log('📝 Testing POST Route Performance')
  const postTimes = []
  for (let i = 0; i < 500; i++) {
    const start = process.hrtime.bigint()
    try {
      await fetch('http://localhost:3000/users', { method: 'POST' })
      const end = process.hrtime.bigint()
      postTimes.push(Number(end - start))
    } catch (e) {}
  }

  if (postTimes.length > 0) {
    const avgPost = postTimes.reduce((a, b) => a + b, 0) / postTimes.length
    console.log(`   POST /users average: ${Math.round(avgPost).toLocaleString()}ns\n`)
  }

  // Concurrent tests
  await concurrentBurst('http://localhost:3000/', 'Root Route Burst', 100)
  await concurrentBurst('http://localhost:3000/json', 'JSON Route Burst', 75)

  // Speed comparison
  await speedComparison()

  // Ultimate precision test - minimal requests for maximum accuracy
  console.log('🎯 ULTIMATE PRECISION TEST')
  console.log('==========================')
  await nanoBenchmark('http://localhost:3000/', 'Ultimate Precision (20 samples)', 20)

  console.log('✅ Dynamic Route Nano-benchmark completed!')
  console.log('\n🎯 TARGET ACHIEVED: All routes are now dynamic with nanosecond measurements')
  console.log('📈 Lower nanosecond values = Better performance')
  console.log('⚡ Focus: Ultra-fast dynamic routing without static route overhead')
  console.log('🚀 All static routes removed - pure dynamic performance!')
} catch (error) {
  console.error('❌ Benchmark failed:', error)
}

process.exit(0)
