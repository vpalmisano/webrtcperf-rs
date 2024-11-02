# webrtcperf-vmaf-utils-js

Node bindings of the `webrtcperf-vmaf-utils` library (https://github.com/vpalmisano/webrtcperf-vmaf-utils).

## Installation
```bash
npm install @vpalmisano/webrtcperf-vmaf-utils
```

## Usage
```js
import { watermark, process } from "@vpalmisano/webrtcperf-vmaf-utils"

await watermark("video/file.mp4", "1")

await process("video/file.mp4")
```
