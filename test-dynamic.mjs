import { tachyon } from './index.js'

const app = tachyon()

// Test dynamic routes
app.get('/regular', (req, res) => {
  res.send('Hello World!')
})

app.post('/users', (req, res) => {
  res.send('User created!')
})

app.get('/json', (req, res) => {
  res.json({ message: 'Hello JSON', timestamp: Date.now() })
})

// Test additional dynamic routes
app.get('/api/status', (req, res) => {
  res.json({ status: 'operational', timestamp: Date.now() })
})

app.put('/users/:id', (req, res) => {
  res.json({ id: 1, updated: true })
})

console.log('🧪 Testing Dynamic Routes Only')
console.log('==============================')
console.log('All static routes removed - testing pure dynamic performance')

// Start server
console.log('Starting server on port 3001...')
app.listen(3001)

// Wait for server to start
await new Promise((resolve) => setTimeout(resolve, 100))

async function testRoute(url, expectedText, description) {
  console.log(`\n📝 Testing: ${description}`)
  console.log(`   URL: ${url}`)

  try {
    const response = await fetch(url)
    const text = await response.text()

    if (text.includes(expectedText)) {
      console.log(`   ✅ SUCCESS: Got "${text}"`)
      return true
    } else {
      console.log(`   ❌ FAILED: Expected "${expectedText}" but got "${text}"`)
      return false
    }
  } catch (error) {
    console.log(`   ❌ ERROR: ${error.message}`)
    return false
  }
}

// Run tests
let passed = 0
let total = 0

// Test dynamic routes
total++
if (await testRoute('http://localhost:3001/regular', 'Hello World', 'Dynamic GET route')) {
  passed++
}

total++
if (await testRoute('http://localhost:3001/json', 'message', 'Dynamic JSON route')) {
  passed++
}

// Test POST route
total++
try {
  console.log(`\n📝 Testing: Dynamic POST route`)
  console.log(`   URL: http://localhost:3001/users`)

  const response = await fetch('http://localhost:3001/users', { method: 'POST' })
  const text = await response.text()

  if (text.includes('User created')) {
    console.log(`   ✅ SUCCESS: Got "${text}"`)
    passed++
  } else {
    console.log(`   ❌ FAILED: Expected "User created" but got "${text}"`)
  }
} catch (error) {
  console.log(`   ❌ ERROR: ${error.message}`)
}

// Test additional dynamic routes
total++
if (await testRoute('http://localhost:3001/api/status', 'operational', 'Dynamic API status route')) {
  passed++
}

// Test PUT route
total++
try {
  console.log(`\n📝 Testing: Dynamic PUT route`)
  console.log(`   URL: http://localhost:3001/users/1`)

  const response = await fetch('http://localhost:3001/users/1', { method: 'PUT' })
  const text = await response.text()

  if (text.includes('updated')) {
    console.log(`   ✅ SUCCESS: Got "${text}"`)
    passed++
  } else {
    console.log(`   ❌ FAILED: Expected "updated" but got "${text}"`)
  }
} catch (error) {
  console.log(`   ❌ ERROR: ${error.message}`)
}

// Test 404
total++
if (await testRoute('http://localhost:3001/notfound', 'Not Found', '404 handling')) {
  passed++
}

console.log(`\n📊 Results: ${passed}/${total} tests passed`)

if (passed === total) {
  console.log('🎉 All tests passed! Dynamic routes are working correctly.')
  console.log('🚀 Static routes completely removed - pure dynamic performance achieved!')
} else {
  console.log('❌ Some tests failed. Check the output above.')
}

process.exit(passed === total ? 0 : 1)
