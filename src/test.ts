import { watermark } from "./index.cjs"

(async () => {
    await watermark(process.argv[2], "1")
})().catch(console.error)
