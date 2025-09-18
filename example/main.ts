import { tachyon } from '../index'

const app = tachyon()
app.get('/', (_, res) => {
  console.log(res[1])
  res[1].send({
    message: 'Hello, world!',
  })
})
app.listen(3333)
