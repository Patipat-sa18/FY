export function fileToBase64(file: File): Promise<string> {
    return new Promise((resolve, reject) => {
        const reader = new FileReader()
        reader.onload = () => {
            const base64 = reader.result as string
            resolve(base64)
        }

        reader.onerror = (e) => {
            reject(e)
        }
        reader.readAsDataURL(file)
    })
}