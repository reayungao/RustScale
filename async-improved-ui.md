VITE v6.4.1  ready in 1055 ms

  ➜  Local:   http://localhost:1420/
     Running DevCommand (`cargo  run --no-default-features --color always --`)
        Info Watching C:\Users\reayu\OneDrive\Desktop\Work Notes\RustScale\app\src-tauri for changes...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running `target\debug\tauri-apprustscale.exe`
2025-12-02T10:30:47.025377Z  INFO ThreadId(01) Logging initialized (Console Only)
2025-12-02T10:30:48.750791Z  INFO ThreadId(16) Loading model from: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx"
2025-12-02T10:30:49.293207Z  INFO ThreadId(16) Model loaded: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx" | Input Type: Float32
2025-12-02T10:30:49.293449Z  INFO ThreadId(16) Detecting scale for: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx"
2025-12-02T10:31:09.278901Z  INFO ThreadId(14) Starting batch job 5eb02019-b2e6-439b-bf46-e60714f46c7d: 35 files
2025-12-02T10:31:09.279205Z  INFO ThreadId(14) Loading model session: SAFMNv3_x4.onnx
2025-12-02T10:31:09.279439Z  INFO ThreadId(14) Cache Hit: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx" 
2025-12-02T10:31:09.279583Z  INFO ThreadId(14) Batch Model Loaded: SAFMNv3_x4.onnx
2025-12-02T10:31:09.280071Z  INFO ThreadId(19) Starting upscale job c7e3fe47-25ce-4ea1-af41-9d6718644e16: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3860.JPG"
2025-12-02T10:31:09.283662Z  INFO ThreadId(19) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:09.283828Z  INFO ThreadId(19) Model Constraint: Fixed input size 512
2025-12-02T10:31:09.283971Z  INFO ThreadId(19) Scaling Match. Running single pass.
2025-12-02T10:31:09.573309Z  INFO ThreadId(19) Pipeline Active | Batch #1 | TPS: 3.5
2025-12-02T10:31:10.432133Z  INFO ThreadId(19) Inference Complete | Total Batches: 4 | Duration: 1.15s | Avg TPS: 3.48
2025-12-02T10:31:10.436267Z  INFO ThreadId(33) Starting upscale job 82220731-c450-4eb8-814b-a22ad90ad0b2: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3870.JPG"
2025-12-02T10:31:10.440418Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:10.440551Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:10.440687Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:10.506718Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.1
2025-12-02T10:31:11.325062Z  INFO ThreadId(19) Post-Processing | Encode: 0.89s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:31:11.388459Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.95s | Avg TPS: 4.22
2025-12-02T10:31:11.392049Z  INFO ThreadId(33) Starting upscale job 5566b9b1-55bf-4a3c-a2ec-3009d3ea55bf: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3880.JPG"
2025-12-02T10:31:11.395827Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:11.396003Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:11.396108Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:11.464543Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 14.6
2025-12-02T10:31:12.260044Z  INFO ThreadId(19) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:12.344891Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.95s | Avg TPS: 4.22
2025-12-02T10:31:12.348195Z  INFO ThreadId(33) Starting upscale job 0b5903ab-eaba-4da0-99a8-78ce3325df48: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3888.JPG"
2025-12-02T10:31:12.352102Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:12.352238Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:12.352335Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:12.417587Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.3
2025-12-02T10:31:13.205380Z  INFO ThreadId(19) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T10:31:13.289465Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.27
2025-12-02T10:31:13.293645Z  INFO ThreadId(33) Starting upscale job 5cbb232b-a378-4ab9-9472-f04261bf11a3: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3890.JPG"
2025-12-02T10:31:13.297577Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:13.297711Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:13.297804Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:13.388365Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 11.0
2025-12-02T10:31:14.184232Z  INFO ThreadId(19) Post-Processing | Encode: 0.89s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:31:14.265521Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.97s | Avg TPS: 4.13
2025-12-02T10:31:14.269456Z  INFO ThreadId(33) Starting upscale job d34a81a0-5aa4-4885-b871-690d49cf3459: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3950.JPG"
2025-12-02T10:31:14.273486Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:14.273613Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:14.273698Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:14.339124Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.3
2025-12-02T10:31:15.136966Z  INFO ThreadId(19) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:15.206231Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.29
2025-12-02T10:31:15.210538Z  INFO ThreadId(33) Starting upscale job 5f7cb0ff-e7d8-432c-8bd3-731d956fd3f6: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4039.JPG"
2025-12-02T10:31:15.214166Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:15.214300Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:15.214381Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:15.277321Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T10:31:16.080425Z  INFO ThreadId(19) Post-Processing | Encode: 0.86s | Metadata+Save: 0.01s | Total: 0.87s
2025-12-02T10:31:16.150231Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.27
2025-12-02T10:31:16.154592Z  INFO ThreadId(33) Starting upscale job b57b7ede-c4ad-457d-a760-529d810295b8: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4043.JPG"
2025-12-02T10:31:16.158296Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:16.158409Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:16.158497Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:16.221736Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.8
2025-12-02T10:31:17.036883Z  INFO ThreadId(19) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T10:31:17.072449Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.38
2025-12-02T10:31:17.076137Z  INFO ThreadId(33) Starting upscale job b7cd4db3-a0a9-4cdf-a3e1-cea2396f567f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4047.JPG"
2025-12-02T10:31:17.080255Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:17.080436Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:17.080532Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:17.145854Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.3
2025-12-02T10:31:17.933857Z  INFO ThreadId(19) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T10:31:18.009401Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T10:31:18.013024Z  INFO ThreadId(33) Starting upscale job ca0ba710-5c8b-49ad-ae92-8474273eb31f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4053.JPG"
2025-12-02T10:31:18.017068Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:18.017556Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:18.017701Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:18.082814Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.4
2025-12-02T10:31:18.864553Z  INFO ThreadId(19) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T10:31:18.935543Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.36
2025-12-02T10:31:18.939367Z  INFO ThreadId(33) Starting upscale job 728619c7-0cc5-4f4e-be52-c39272b0ebee: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4056.JPG"
2025-12-02T10:31:18.944331Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:18.944466Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:18.944570Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:19.006185Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.2
2025-12-02T10:31:19.819944Z  INFO ThreadId(19) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T10:31:19.853052Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.40
2025-12-02T10:31:19.857048Z  INFO ThreadId(33) Starting upscale job 5a652ff2-114d-4bfb-a538-dbf5da3d2f51: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4070.JPG"
2025-12-02T10:31:19.861147Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:19.861327Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:19.861437Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:19.923066Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.2
2025-12-02T10:31:20.723693Z  INFO ThreadId(19) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:20.777116Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.37
2025-12-02T10:31:20.780850Z  INFO ThreadId(33) Starting upscale job a4da69e3-98c4-428f-bcb0-977c10badba1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4071.JPG"
2025-12-02T10:31:20.784734Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:20.784895Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:20.785012Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:20.847453Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.0
2025-12-02T10:31:21.654929Z  INFO ThreadId(19) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:21.713315Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T10:31:21.716885Z  INFO ThreadId(33) Starting upscale job 82c0100a-070c-475e-957a-e6593693223d: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4073.JPG"
2025-12-02T10:31:21.721016Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:21.721141Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:21.721217Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:21.786583Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.3
2025-12-02T10:31:22.607784Z  INFO ThreadId(19) Post-Processing | Encode: 0.89s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:31:22.644907Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.33
2025-12-02T10:31:22.648549Z  INFO ThreadId(33) Starting upscale job 6f93dd3b-e1dd-4357-bea4-b38cbd48fbcf: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4107.JPG"
2025-12-02T10:31:22.652450Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:22.652586Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:22.652688Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:22.713446Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.5
2025-12-02T10:31:23.500679Z  INFO ThreadId(19) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T10:31:23.574920Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.34
2025-12-02T10:31:23.578375Z  INFO ThreadId(33) Starting upscale job 633426d7-08b2-4441-b4eb-37a87b196ab0: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4126.JPG"
2025-12-02T10:31:23.581913Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:23.582044Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:23.582124Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:23.642804Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.5
2025-12-02T10:31:24.470225Z  INFO ThreadId(19) Post-Processing | Encode: 0.89s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:31:24.510288Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T10:31:24.514021Z  INFO ThreadId(33) Starting upscale job 24157e43-bcf2-4002-a391-e3f109602890: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4136.JPG"
2025-12-02T10:31:24.517599Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:24.517724Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:24.517862Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:24.581239Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.8
2025-12-02T10:31:25.394423Z  INFO ThreadId(19) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T10:31:25.444552Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.32
2025-12-02T10:31:25.448519Z  INFO ThreadId(33) Starting upscale job 9bafb426-e8e0-465a-b621-864cc44c0a46: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4174.JPG"
2025-12-02T10:31:25.453620Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:25.453770Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:25.453888Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:25.516073Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.1
2025-12-02T10:31:26.346987Z  INFO ThreadId(19) Post-Processing | Encode: 0.90s | Metadata+Save: 0.00s | Total: 0.90s
2025-12-02T10:31:26.382647Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T10:31:26.386872Z  INFO ThreadId(33) Starting upscale job e1787cc0-2f09-466c-b5c4-bce072f26481: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4178.JPG"
2025-12-02T10:31:26.391451Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:26.391581Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:26.391699Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:26.455590Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.7
2025-12-02T10:31:27.306353Z  INFO ThreadId(19) Post-Processing | Encode: 0.89s | Metadata+Save: 0.03s | Total: 0.92s
2025-12-02T10:31:27.310158Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.36
2025-12-02T10:31:27.313555Z  INFO ThreadId(33) Starting upscale job c9484e9d-8e58-4fb6-9f9e-bdabf7b57e8e: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4180.JPG"
2025-12-02T10:31:27.317345Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:27.317463Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:27.317647Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:27.382161Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.5
2025-12-02T10:31:28.199764Z  INFO ThreadId(19) Post-Processing | Encode: 0.88s | Metadata+Save: 0.01s | Total: 0.89s
2025-12-02T10:31:28.236206Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.35
2025-12-02T10:31:28.239913Z  INFO ThreadId(33) Starting upscale job 19370e05-3b96-4453-9bdb-7df76a6e4a5f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4184.JPG"
2025-12-02T10:31:28.243647Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:28.243807Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:28.243915Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:28.306096Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.1
2025-12-02T10:31:29.108767Z  INFO ThreadId(19) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:29.167527Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.33
2025-12-02T10:31:29.170874Z  INFO ThreadId(33) Starting upscale job 9cc8d622-784d-410e-b49a-ae34206b35fa: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4190.JPG"
2025-12-02T10:31:29.174577Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:29.174681Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:29.174770Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:29.239651Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.4
2025-12-02T10:31:30.090248Z  INFO ThreadId(19) Post-Processing | Encode: 0.89s | Metadata+Save: 0.03s | Total: 0.92s
2025-12-02T10:31:30.093808Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.35
2025-12-02T10:31:30.097011Z  INFO ThreadId(33) Starting upscale job bd3439d2-9d35-462b-9ea5-51589db6f8d0: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4215.JPG"
2025-12-02T10:31:30.101990Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:30.102186Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:30.102298Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:30.163562Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.3
2025-12-02T10:31:30.954410Z  INFO ThreadId(19) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T10:31:31.024344Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.34
2025-12-02T10:31:31.028120Z  INFO ThreadId(33) Starting upscale job da92df89-74a9-4e6e-95e0-8423a4ac29f4: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4327.JPG"
2025-12-02T10:31:31.031882Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:31.032016Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:31.032120Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:31.098245Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.1
2025-12-02T10:31:31.901731Z  INFO ThreadId(19) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:31.972204Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.25
2025-12-02T10:31:31.975273Z  INFO ThreadId(33) Starting upscale job 52a650bf-0ea0-4a45-a1e2-c0ed7ff45285: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4331.JPG"
2025-12-02T10:31:31.978897Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:31.979077Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:31.979180Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:32.062773Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 12.0
2025-12-02T10:31:32.866491Z  INFO ThreadId(19) Post-Processing | Encode: 0.89s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:31:32.944426Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.97s | Avg TPS: 4.14
2025-12-02T10:31:32.947871Z  INFO ThreadId(33) Starting upscale job cd9dd5f3-5122-4821-a2f0-4904d85b9b69: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4359.JPG"
2025-12-02T10:31:32.951717Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:32.951861Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:32.952004Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:33.018678Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.0
2025-12-02T10:31:33.827992Z  INFO ThreadId(19) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T10:31:33.898055Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.95s | Avg TPS: 4.23
2025-12-02T10:31:33.901761Z  INFO ThreadId(33) Starting upscale job 3ad030e3-a45a-42c9-9565-ffb8825f27d5: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4385.JPG"
2025-12-02T10:31:33.905403Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:33.905530Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:33.905635Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:33.970686Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.4
2025-12-02T10:31:34.789558Z  INFO ThreadId(19) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:31:34.845510Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.26
2025-12-02T10:31:34.849013Z  INFO ThreadId(33) Starting upscale job 50e8b069-73d3-4c2e-a7ab-db5cacb4f100: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4407.JPG"
2025-12-02T10:31:34.853103Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:34.853227Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:34.853319Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:34.915496Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.1
2025-12-02T10:31:35.720903Z  INFO ThreadId(19) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:35.789895Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.27
2025-12-02T10:31:35.793722Z  INFO ThreadId(33) Starting upscale job 86c8a401-77e5-4c79-8064-801de9074799: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4421.JPG"
2025-12-02T10:31:35.797869Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:35.798016Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:35.798118Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:35.863476Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.3
2025-12-02T10:31:36.656538Z  INFO ThreadId(19) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T10:31:36.734596Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.27
2025-12-02T10:31:36.738473Z  INFO ThreadId(33) Starting upscale job 026c71fc-f833-4d2b-92ef-3800e976668b: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4443.JPG"
2025-12-02T10:31:36.742074Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:36.742187Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:36.742277Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:36.808680Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.1
2025-12-02T10:31:37.609177Z  INFO ThreadId(19) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:37.698309Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.96s | Avg TPS: 4.18
2025-12-02T10:31:37.702596Z  INFO ThreadId(33) Starting upscale job 1fe4c1f8-8776-4304-81b7-c6b8315f0604: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4445.JPG"
2025-12-02T10:31:37.706116Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:37.706258Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:37.706376Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:37.770719Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.5
2025-12-02T10:31:38.583525Z  INFO ThreadId(19) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T10:31:38.645364Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.26
2025-12-02T10:31:38.648515Z  INFO ThreadId(33) Starting upscale job 74f2174b-b93e-4a05-9449-679b35581085: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4486.JPG"
2025-12-02T10:31:38.652699Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:38.652865Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:38.652953Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:38.715162Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.1
2025-12-02T10:31:39.523747Z  INFO ThreadId(19) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T10:31:39.588477Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.28
2025-12-02T10:31:39.592139Z  INFO ThreadId(33) Starting upscale job 5ebf5e79-a45d-417c-a929-a8cd31064028: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4502.JPG"
2025-12-02T10:31:39.595885Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:39.596007Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:39.596108Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:39.657661Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 16.2
2025-12-02T10:31:40.481787Z  INFO ThreadId(19) Post-Processing | Encode: 0.89s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:31:40.529247Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.29
2025-12-02T10:31:40.532810Z  INFO ThreadId(33) Starting upscale job 84f86675-3671-4d50-8336-5fa25f847d17: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4541.JPG"
2025-12-02T10:31:40.537147Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:40.537304Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:40.537413Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:40.606607Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 14.5
2025-12-02T10:31:41.404322Z  INFO ThreadId(19) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T10:31:41.488180Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.95s | Avg TPS: 4.21
2025-12-02T10:31:41.491603Z  INFO ThreadId(33) Starting upscale job 45b8db22-5d85-4302-9da7-bb1e396d09cc: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4645.JPG"
2025-12-02T10:31:41.495238Z  INFO ThreadId(33) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:31:41.495379Z  INFO ThreadId(33) Model Constraint: Fixed input size 512
2025-12-02T10:31:41.495511Z  INFO ThreadId(33) Scaling Match. Running single pass.
2025-12-02T10:31:41.558377Z  INFO ThreadId(33) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T10:31:42.379093Z  INFO ThreadId(19) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:31:42.414738Z  INFO ThreadId(33) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.35
2025-12-02T10:31:43.241405Z  INFO ThreadId(19) Post-Processing | Encode: 0.82s | Metadata+Save: 0.00s | Total: 0.82s