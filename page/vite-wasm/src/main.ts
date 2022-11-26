import hello_wasm from '../../../pkg/hello_wasm'

const button = document.getElementById('select') as HTMLButtonElement
const file = document.getElementById('file') as HTMLInputElement

button.addEventListener('fileImport', () => {
  console.log('点击了选择按钮')
  file.click()
})
