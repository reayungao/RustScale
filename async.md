Local:   http://localhost:1420/
     Running DevCommand (`cargo  run --no-default-features --color always --`)
        Info Watching C:\Users\reayu\OneDrive\Desktop\Work Notes\RustScale\app\src-tauri for changes...
   Compiling tauri-apprustscale v0.1.0 (C:\Users\reayu\OneDrive\Desktop\Work Notes\RustScale\app\src-tauri)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 22.02s
     Running `target\debug\tauri-apprustscale.exe`
2025-12-02T09:58:19.738759Z  INFO ThreadId(01) Logging initialized (Console Only)
2025-12-02T09:58:21.328292Z  INFO ThreadId(16) Loading model from: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx"
2025-12-02T09:58:21.926349Z  INFO ThreadId(16) Model loaded: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx" | Input Type: Float32
2025-12-02T09:58:21.926533Z  INFO ThreadId(16) Detecting scale for: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx"
2025-12-02T09:58:27.459604Z  INFO ThreadId(14) Starting batch job 1de81dd3-0e3f-4b1d-bd47-0b26a1e24a57: 35 files
2025-12-02T09:58:27.459911Z  INFO ThreadId(14) Loading model session: SAFMNv3_x4.onnx
2025-12-02T09:58:27.460157Z  INFO ThreadId(14) Cache Hit: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx" 
2025-12-02T09:58:27.460294Z  INFO ThreadId(14) Batch Model Loaded: SAFMNv3_x4.onnx
2025-12-02T09:58:27.460533Z  INFO ThreadId(16) Starting upscale job ed810783-b34d-4604-9d1b-f264238f1496: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3860.JPG"
2025-12-02T09:58:27.467448Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:27.467649Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:58:27.467838Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:58:27.802088Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 3.0
2025-12-02T09:58:28.665131Z  INFO ThreadId(16) Inference Complete | Total Batches: 4 | Duration: 1.20s | Avg TPS: 3.34
2025-12-02T09:58:28.668376Z  INFO ThreadId(32) Starting upscale job 7b8a8273-0f4c-4528-b79f-e6e02f0e15f6: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3870.JPG"
2025-12-02T09:58:28.672118Z  INFO ThreadId(32) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:28.672245Z  INFO ThreadId(32) Model Constraint: Fixed input size 512
2025-12-02T09:58:28.672358Z  INFO ThreadId(32) Scaling Match. Running single pass.
2025-12-02T09:58:28.735957Z  INFO ThreadId(32) Pipeline Active | Batch #1 | TPS: 15.7
2025-12-02T09:58:29.600642Z  INFO ThreadId(32) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T09:58:29.604754Z  INFO ThreadId(34) Starting upscale job 90ff49c1-fc11-4a40-b8e7-0a72b17d4498: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3880.JPG"
2025-12-02T09:58:29.605777Z  INFO ThreadId(16) Post-Processing | Encode: 0.94s | Metadata+Save: 0.00s | Total: 0.94s
2025-12-02T09:58:29.608189Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:29.608316Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:29.608408Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:29.669481Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.4
2025-12-02T09:58:30.534461Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.32
2025-12-02T09:58:30.538674Z  INFO ThreadId(34) Starting upscale job cad6f85d-0fff-4cf0-a795-ee3861d7191d: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3888.JPG"
2025-12-02T09:58:30.543476Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:30.543636Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:30.543755Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:30.549923Z  INFO ThreadId(32) Post-Processing | Encode: 0.94s | Metadata+Save: 0.00s | Total: 0.95s
2025-12-02T09:58:30.606060Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.1
2025-12-02T09:58:31.419910Z  INFO ThreadId(16) Post-Processing | Encode: 0.88s | Metadata+Save: 0.01s | Total: 0.88s
2025-12-02T09:58:31.456035Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.38
2025-12-02T09:58:31.460672Z  INFO ThreadId(16) Starting upscale job 5bd28d2d-3413-4c04-873e-b3c7647e36c8: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3890.JPG"
2025-12-02T09:58:31.464778Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:31.464930Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:58:31.465072Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:58:31.532307Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 14.9
2025-12-02T09:58:32.393084Z  INFO ThreadId(16) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T09:58:32.394982Z  INFO ThreadId(32) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T09:58:32.397299Z  INFO ThreadId(32) Starting upscale job d2fcefb1-b010-4fec-9c7f-451c1ac52fa1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3950.JPG"
2025-12-02T09:58:32.401048Z  INFO ThreadId(32) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:32.401153Z  INFO ThreadId(32) Model Constraint: Fixed input size 512
2025-12-02T09:58:32.401256Z  INFO ThreadId(32) Scaling Match. Running single pass.
2025-12-02T09:58:32.464199Z  INFO ThreadId(32) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T09:58:33.290604Z  INFO ThreadId(34) Post-Processing | Encode: 0.89s | Metadata+Save: 0.01s | Total: 0.89s
2025-12-02T09:58:33.323037Z  INFO ThreadId(32) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.34
2025-12-02T09:58:33.327588Z  INFO ThreadId(34) Starting upscale job 9bb23d96-8b05-4a4f-a92c-cd56df384007: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4039.JPG"
2025-12-02T09:58:33.331484Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:33.331613Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:33.331725Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:33.394955Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 15.8
2025-12-02T09:58:34.233524Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.44
2025-12-02T09:58:34.237232Z  INFO ThreadId(34) Starting upscale job d49f5b6f-c793-455d-b8b0-186ab9f32ac7: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4043.JPG"
2025-12-02T09:58:34.240720Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:34.240838Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:34.240941Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:34.268633Z  INFO ThreadId(16) Post-Processing | Encode: 0.94s | Metadata+Save: 0.00s | Total: 0.94s
2025-12-02T09:58:34.307080Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 15.1
2025-12-02T09:58:35.125737Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.88s | Avg TPS: 4.52
2025-12-02T09:58:35.129772Z  INFO ThreadId(34) Starting upscale job f426e290-2463-487d-bb03-0d12be80ce42: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4047.JPG"
2025-12-02T09:58:35.134687Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:35.134847Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:35.134936Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:35.144484Z  INFO ThreadId(32) Post-Processing | Encode: 0.90s | Metadata+Save: 0.00s | Total: 0.91s
2025-12-02T09:58:35.195193Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.6
2025-12-02T09:58:36.089857Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.95s | Avg TPS: 4.19
2025-12-02T09:58:36.093748Z  INFO ThreadId(34) Starting upscale job 8cc7c23d-bd9f-4dc0-8b25-a1e26bc8f5f6: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4053.JPG"
2025-12-02T09:58:36.097514Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:36.097654Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:36.097750Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:36.129305Z  INFO ThreadId(16) Post-Processing | Encode: 1.00s | Metadata+Save: 0.00s | Total: 1.00s
2025-12-02T09:58:36.166979Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 14.4
2025-12-02T09:58:37.028611Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.30
2025-12-02T09:58:37.032657Z  INFO ThreadId(34) Starting upscale job 1c40a3ff-77aa-475c-9567-fb7596ca8d18: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4056.JPG"
2025-12-02T09:58:37.036829Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:37.036950Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:37.037034Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:37.037043Z  INFO ThreadId(32) Post-Processing | Encode: 0.94s | Metadata+Save: 0.00s | Total: 0.94s
2025-12-02T09:58:37.103443Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 15.1
2025-12-02T09:58:37.952767Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.37
2025-12-02T09:58:37.956808Z  INFO ThreadId(34) Starting upscale job e9b83930-e807-4a94-a8aa-5c64d22e791d: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4070.JPG"
2025-12-02T09:58:37.961549Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:37.961696Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:37.961827Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:37.969671Z  INFO ThreadId(16) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.94s
2025-12-02T09:58:38.031000Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 14.5
2025-12-02T09:58:38.875063Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.38
2025-12-02T09:58:38.879156Z  INFO ThreadId(34) Starting upscale job 2202d8b7-c3f9-47f0-8cea-0e9f2e7dba6e: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4071.JPG"
2025-12-02T09:58:38.883273Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:38.883423Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:38.883522Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:38.900428Z  INFO ThreadId(32) Post-Processing | Encode: 0.94s | Metadata+Save: 0.00s | Total: 0.94s
2025-12-02T09:58:38.946418Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T09:58:39.794708Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.39
2025-12-02T09:58:39.798463Z  INFO ThreadId(34) Starting upscale job 78889cbd-d57b-4420-92ad-24ac375b64a7: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4073.JPG"
2025-12-02T09:58:39.803160Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:39.803307Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:39.803445Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:39.808816Z  INFO ThreadId(16) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T09:58:39.872029Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 14.6
2025-12-02T09:58:40.724594Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.34
2025-12-02T09:58:40.727791Z  INFO ThreadId(32) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T09:58:40.728990Z  INFO ThreadId(34) Starting upscale job f41e4acb-fbcf-4146-98ad-068d0210cf96: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4107.JPG"
2025-12-02T09:58:40.732564Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:40.732679Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:40.732776Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:40.794430Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.2
2025-12-02T09:58:41.592916Z  INFO ThreadId(16) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:58:41.623464Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.89s | Avg TPS: 4.49
2025-12-02T09:58:41.626832Z  INFO ThreadId(16) Starting upscale job de034a3f-462c-4cbb-b4b9-b2cd27c3abf1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4126.JPG"
2025-12-02T09:58:41.630849Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:41.631008Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:58:41.631096Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:58:41.692588Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.3
2025-12-02T09:58:42.495988Z  INFO ThreadId(32) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T09:58:42.533955Z  INFO ThreadId(16) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.43
2025-12-02T09:58:42.538325Z  INFO ThreadId(32) Starting upscale job 498a028b-d115-49ae-be24-a37c020ecb25: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4136.JPG"
2025-12-02T09:58:42.542007Z  INFO ThreadId(32) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:42.542121Z  INFO ThreadId(32) Model Constraint: Fixed input size 512
2025-12-02T09:58:42.542221Z  INFO ThreadId(32) Scaling Match. Running single pass.
2025-12-02T09:58:42.599654Z  INFO ThreadId(32) Pipeline Active | Batch #1 | TPS: 17.4
2025-12-02T09:58:43.445973Z  INFO ThreadId(32) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.43
2025-12-02T09:58:43.449761Z  INFO ThreadId(32) Starting upscale job cd0975a7-23b4-4547-8f5c-5f9891add6df: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4174.JPG"
2025-12-02T09:58:43.453367Z  INFO ThreadId(32) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:43.453476Z  INFO ThreadId(32) Model Constraint: Fixed input size 512
2025-12-02T09:58:43.453586Z  INFO ThreadId(32) Scaling Match. Running single pass.
2025-12-02T09:58:43.466485Z  INFO ThreadId(34) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T09:58:43.518707Z  INFO ThreadId(32) Pipeline Active | Batch #1 | TPS: 15.4
2025-12-02T09:58:44.336439Z  INFO ThreadId(16) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T09:58:44.386405Z  INFO ThreadId(32) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.29
2025-12-02T09:58:44.389662Z  INFO ThreadId(16) Starting upscale job 4ef40e2b-4d0c-4867-81b4-6462a304f74d: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4178.JPG"
2025-12-02T09:58:44.393829Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:44.393978Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:58:44.394091Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:58:44.456934Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T09:58:45.295750Z  INFO ThreadId(34) Post-Processing | Encode: 0.90s | Metadata+Save: 0.00s | Total: 0.91s
2025-12-02T09:58:45.328794Z  INFO ThreadId(16) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.28
2025-12-02T09:58:45.332428Z  INFO ThreadId(34) Starting upscale job 91c6299b-fce1-4c38-acb9-104fe250e1f1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4180.JPG"
2025-12-02T09:58:45.336423Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:45.336550Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:45.336631Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:45.401390Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 15.4
2025-12-02T09:58:46.250136Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.38
2025-12-02T09:58:46.253867Z  INFO ThreadId(34) Starting upscale job d34dd1eb-14f5-47dd-9a05-a584402909b8: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4184.JPG"
2025-12-02T09:58:46.257569Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:46.257670Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:46.257758Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:46.261534Z  INFO ThreadId(32) Post-Processing | Encode: 0.92s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T09:58:46.320767Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T09:58:47.155778Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.45
2025-12-02T09:58:47.157434Z  INFO ThreadId(16) Post-Processing | Encode: 0.90s | Metadata+Save: 0.01s | Total: 0.90s
2025-12-02T09:58:47.159140Z  INFO ThreadId(16) Starting upscale job e65e9dca-7f23-4e90-86f2-7eac7116e63f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4190.JPG"
2025-12-02T09:58:47.162826Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:47.162964Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:58:47.163081Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:58:47.220770Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 17.3
2025-12-02T09:58:48.015858Z  INFO ThreadId(32) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:58:48.074775Z  INFO ThreadId(16) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.39
2025-12-02T09:58:48.078161Z  INFO ThreadId(32) Starting upscale job 423968ec-561a-47ac-a326-68973002f1aa: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4215.JPG"
2025-12-02T09:58:48.082070Z  INFO ThreadId(32) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:48.082218Z  INFO ThreadId(32) Model Constraint: Fixed input size 512
2025-12-02T09:58:48.082328Z  INFO ThreadId(32) Scaling Match. Running single pass.
2025-12-02T09:58:48.140397Z  INFO ThreadId(32) Pipeline Active | Batch #1 | TPS: 17.2
2025-12-02T09:58:48.932064Z  INFO ThreadId(34) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:58:48.980175Z  INFO ThreadId(32) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.46
2025-12-02T09:58:48.983738Z  INFO ThreadId(34) Starting upscale job 9024c650-f49d-40f5-bce9-580982f7154a: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4327.JPG"
2025-12-02T09:58:48.987312Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:48.987490Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:48.987584Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:49.047440Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.7
2025-12-02T09:58:49.859272Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.02s | Total: 0.88s
2025-12-02T09:58:49.862830Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.88s | Avg TPS: 4.57
2025-12-02T09:58:49.866402Z  INFO ThreadId(16) Starting upscale job 79456d09-ad3e-4bfa-b481-5610a7dd4e03: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4331.JPG"
2025-12-02T09:58:49.870149Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:49.870302Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:58:49.870384Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:58:49.929311Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 17.0
2025-12-02T09:58:50.734420Z  INFO ThreadId(32) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T09:58:50.784144Z  INFO ThreadId(16) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.38
2025-12-02T09:58:50.787749Z  INFO ThreadId(32) Starting upscale job d19852d7-6993-445a-b12e-992a22ce6782: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4359.JPG"
2025-12-02T09:58:50.791494Z  INFO ThreadId(32) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:50.791595Z  INFO ThreadId(32) Model Constraint: Fixed input size 512
2025-12-02T09:58:50.791719Z  INFO ThreadId(32) Scaling Match. Running single pass.
2025-12-02T09:58:50.849933Z  INFO ThreadId(32) Pipeline Active | Batch #1 | TPS: 17.2
2025-12-02T09:58:51.657962Z  INFO ThreadId(34) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T09:58:51.691394Z  INFO ThreadId(32) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.45
2025-12-02T09:58:51.695241Z  INFO ThreadId(34) Starting upscale job ef8455d0-9a2d-4ee2-9d3f-8e580d653582: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4385.JPG"
2025-12-02T09:58:51.698545Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:51.698668Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:51.698772Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:51.756762Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 17.2
2025-12-02T09:58:52.565049Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.87s | Avg TPS: 4.62
2025-12-02T09:58:52.569202Z  INFO ThreadId(34) Starting upscale job eda96a8e-94d6-4e93-be04-3e5d865542b1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4407.JPG"
2025-12-02T09:58:52.572763Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:52.572894Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:52.573003Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:52.624589Z  INFO ThreadId(16) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T09:58:52.634220Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.3
2025-12-02T09:58:53.452762Z  INFO ThreadId(32) Post-Processing | Encode: 0.87s | Metadata+Save: 0.01s | Total: 0.88s
2025-12-02T09:58:53.456277Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.88s | Avg TPS: 4.53
2025-12-02T09:58:53.459274Z  INFO ThreadId(32) Starting upscale job 93105cc0-e27b-472c-9540-ed85a1e45a0a: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4421.JPG"
2025-12-02T09:58:53.463244Z  INFO ThreadId(32) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:53.463395Z  INFO ThreadId(32) Model Constraint: Fixed input size 512
2025-12-02T09:58:53.463480Z  INFO ThreadId(32) Scaling Match. Running single pass.
2025-12-02T09:58:53.523070Z  INFO ThreadId(32) Pipeline Active | Batch #1 | TPS: 16.8
2025-12-02T09:58:54.322692Z  INFO ThreadId(16) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:58:54.397683Z  INFO ThreadId(32) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.28
2025-12-02T09:58:54.401003Z  INFO ThreadId(16) Starting upscale job b8448387-9a34-4088-9997-0a1bbc9860a6: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4443.JPG"
2025-12-02T09:58:54.404580Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:54.404695Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:58:54.404793Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:58:54.463647Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 17.0
2025-12-02T09:58:55.265454Z  INFO ThreadId(34) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:58:55.312376Z  INFO ThreadId(16) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.41
2025-12-02T09:58:55.316027Z  INFO ThreadId(34) Starting upscale job 240f1420-8e78-46d1-8cd6-6aac7da521b7: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4445.JPG"
2025-12-02T09:58:55.320909Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:55.321102Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:55.321208Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:55.381601Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.6
2025-12-02T09:58:56.178791Z  INFO ThreadId(32) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:58:56.209682Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.89s | Avg TPS: 4.50
2025-12-02T09:58:56.213344Z  INFO ThreadId(32) Starting upscale job e495bfaa-e9bc-49f5-b021-5baff4ff17ec: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4486.JPG"
2025-12-02T09:58:56.217039Z  INFO ThreadId(32) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:56.217163Z  INFO ThreadId(32) Model Constraint: Fixed input size 512
2025-12-02T09:58:56.217290Z  INFO ThreadId(32) Scaling Match. Running single pass.
2025-12-02T09:58:56.275766Z  INFO ThreadId(32) Pipeline Active | Batch #1 | TPS: 17.1
2025-12-02T09:58:57.112366Z  INFO ThreadId(16) Post-Processing | Encode: 0.87s | Metadata+Save: 0.03s | Total: 0.90s
2025-12-02T09:58:57.116204Z  INFO ThreadId(32) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.45
2025-12-02T09:58:57.120069Z  INFO ThreadId(16) Starting upscale job a60e3129-a7ea-4d59-ad12-11cd06c6f0de: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4502.JPG"
2025-12-02T09:58:57.123728Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:57.123857Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:58:57.123978Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:58:57.182707Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 17.0
2025-12-02T09:58:58.023920Z  INFO ThreadId(34) Post-Processing | Encode: 0.90s | Metadata+Save: 0.01s | Total: 0.90s
2025-12-02T09:58:58.027453Z  INFO ThreadId(16) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.43
2025-12-02T09:58:58.030778Z  INFO ThreadId(34) Starting upscale job 82ebea8c-06f3-425a-a4d7-ead72ea563c0: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4541.JPG"
2025-12-02T09:58:58.034769Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:58.034904Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:58.035058Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:58.094474Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.8
2025-12-02T09:58:58.912675Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.88s | Avg TPS: 4.56
2025-12-02T09:58:58.916112Z  INFO ThreadId(34) Starting upscale job 9da2982e-cd7c-4cfe-8c60-3bbfdc171ac1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4645.JPG"
2025-12-02T09:58:58.920267Z  INFO ThreadId(34) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:58:58.920409Z  INFO ThreadId(34) Model Constraint: Fixed input size 512
2025-12-02T09:58:58.920498Z  INFO ThreadId(34) Scaling Match. Running single pass.
2025-12-02T09:58:58.933552Z  INFO ThreadId(32) Post-Processing | Encode: 0.90s | Metadata+Save: 0.00s | Total: 0.90s
2025-12-02T09:58:58.980947Z  INFO ThreadId(34) Pipeline Active | Batch #1 | TPS: 16.5
2025-12-02T09:58:59.803224Z  INFO ThreadId(34) Inference Complete | Total Batches: 4 | Duration: 0.88s | Avg TPS: 4.53
2025-12-02T09:58:59.806353Z  INFO ThreadId(16) Post-Processing | Encode: 0.89s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T09:59:00.611312Z  INFO ThreadId(32) Post-Processing | Encode: 0.80s | Metadata+Save: 0.00s | Total: 0.80s