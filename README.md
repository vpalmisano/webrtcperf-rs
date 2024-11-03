# @vpalmisano/webrtcperf-rs
Rust utilities for webrtcperf (https://github.com/vpalmisano/@vpalmisano/webrtcperf).

## Installation
```bash
npm install @vpalmisano/webrtcperf-rs
```

## Usage
```js
import { watermark, process } from "@vpalmisano/webrtcperf-rs"

await watermark("video/file.mp4", "1")

await process("video/file.mp4")
```
