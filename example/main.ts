import { tachyon } from '../index'

const app = tachyon();
app.get('/', () => {})
app.listen(3333)