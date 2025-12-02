VITE v6.4.1  ready in 998 ms

  ➜  Local:   http://localhost:1420/
     Running DevCommand (`cargo  run --no-default-features --color always --`)
        Info Watching C:\Users\reayu\OneDrive\Desktop\Work Notes\RustScale\app\src-tauri for changes...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running `target\debug\tauri-apprustscale.exe`
2025-12-02T09:42:11.927036Z  INFO ThreadId(01) Logging initialized (Console Only)
2025-12-02T09:42:13.730988Z  INFO ThreadId(16) Loading model from: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx"
2025-12-02T09:42:14.270640Z  INFO ThreadId(16) Model loaded: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx" | Input Type: Float32
2025-12-02T09:42:14.270908Z  INFO ThreadId(16) Detecting scale for: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx"
2025-12-02T09:42:23.935398Z  INFO ThreadId(14) Starting batch job 21f87132-2d28-4757-9cf4-5b3a0ebdab6e: 35 files
2025-12-02T09:42:23.935686Z  INFO ThreadId(14) Loading model session: SAFMNv3_x4.onnx
2025-12-02T09:42:23.935894Z  INFO ThreadId(14) Cache Hit: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx" 
2025-12-02T09:42:23.936003Z  INFO ThreadId(14) Batch Model Loaded: SAFMNv3_x4.onnx
2025-12-02T09:42:23.936199Z  INFO ThreadId(16) Starting upscale job 8efeb827-663b-4bee-9267-dbe1b599db2b: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3860.JPG"
2025-12-02T09:42:23.939554Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:23.939699Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:23.939776Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:24.263738Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 3.1
2025-12-02T09:42:26.095320Z  INFO ThreadId(16) Post-Processing | Encode: 0.83s | Metadata+Save: 0.00s | Total: 0.83s
2025-12-02T09:42:26.095493Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 2.16s | Avg TPS: 1.86
2025-12-02T09:42:26.099629Z  INFO ThreadId(16) Starting upscale job f5324d2d-9525-4822-907b-337a0c7e66df: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3870.JPG"
2025-12-02T09:42:26.103270Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:26.103385Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:26.103475Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:26.180163Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 13.0
2025-12-02T09:42:27.934298Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:42:27.934454Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.83s | Avg TPS: 2.18
2025-12-02T09:42:27.938107Z  INFO ThreadId(16) Starting upscale job 3a17e9d0-7406-4cc4-8628-291198bae9b6: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3880.JPG"
2025-12-02T09:42:27.941448Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:27.941586Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:27.941688Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:28.007815Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.1
2025-12-02T09:42:29.744028Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:42:29.744187Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.80s | Avg TPS: 2.22
2025-12-02T09:42:29.747959Z  INFO ThreadId(16) Starting upscale job aa530a3d-ab1b-4c48-a1e0-1ae5b578577f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3888.JPG"
2025-12-02T09:42:29.751696Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:29.751830Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:29.751915Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:29.817391Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.3
2025-12-02T09:42:31.546618Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:42:31.546774Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.79s | Avg TPS: 2.23
2025-12-02T09:42:31.551169Z  INFO ThreadId(16) Starting upscale job 99aaaed0-83e1-4f12-9f63-949043294b84: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3890.JPG"
2025-12-02T09:42:31.554743Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:31.554852Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:31.554938Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:31.617873Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T09:42:33.347784Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.00s | Total: 0.84s
2025-12-02T09:42:33.347955Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.79s | Avg TPS: 2.23
2025-12-02T09:42:33.352086Z  INFO ThreadId(16) Starting upscale job 97f71952-2bd9-455a-bcc3-73943e5b6a8f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3950.JPG"
2025-12-02T09:42:33.355561Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:33.355675Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:33.355767Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:33.415919Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.6
2025-12-02T09:42:35.165611Z  INFO ThreadId(16) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T09:42:35.165779Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.81s | Avg TPS: 2.21
2025-12-02T09:42:35.169677Z  INFO ThreadId(16) Starting upscale job 9b9287da-6804-4d33-8f50-02fb6b48a6f4: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4039.JPG"
2025-12-02T09:42:35.173199Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:35.173367Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:35.173504Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:35.239566Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.1
2025-12-02T09:42:36.987921Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:42:36.988104Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.81s | Avg TPS: 2.20
2025-12-02T09:42:36.992227Z  INFO ThreadId(16) Starting upscale job 51f4ab71-ea99-45c5-9b53-26f117b0be92: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4043.JPG"
2025-12-02T09:42:36.995906Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:36.996005Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:36.996094Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:37.058971Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T09:42:38.795381Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:42:38.795547Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.80s | Avg TPS: 2.22
2025-12-02T09:42:38.799565Z  INFO ThreadId(16) Starting upscale job dbb4410e-e556-4445-b718-42240248f7db: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4047.JPG"
2025-12-02T09:42:38.803150Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:38.803262Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:38.803355Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:38.874578Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 14.0
2025-12-02T09:42:40.584990Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.00s | Total: 0.84s
2025-12-02T09:42:40.585138Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.78s | Avg TPS: 2.24
2025-12-02T09:42:40.588692Z  INFO ThreadId(16) Starting upscale job 68cf0c3c-98fd-4b7d-bcf1-d2051615c4b5: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4053.JPG"
2025-12-02T09:42:40.592155Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:40.592349Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:40.592452Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:40.660398Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 14.7
2025-12-02T09:42:42.386810Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:42:42.386974Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.79s | Avg TPS: 2.23
2025-12-02T09:42:42.391056Z  INFO ThreadId(16) Starting upscale job 29b3b475-1f9a-4ecf-be35-5c0e18625153: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4056.JPG"
2025-12-02T09:42:42.394480Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:42.394637Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:42.394754Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:42.458735Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.6
2025-12-02T09:42:44.169093Z  INFO ThreadId(16) Post-Processing | Encode: 0.83s | Metadata+Save: 0.00s | Total: 0.84s
2025-12-02T09:42:44.169217Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.77s | Avg TPS: 2.25
2025-12-02T09:42:44.172997Z  INFO ThreadId(16) Starting upscale job ba452caa-5f0a-48d4-b4e5-b72ad49dd549: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4070.JPG"
2025-12-02T09:42:44.176414Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:44.176560Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:44.176668Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:44.243594Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 14.9
2025-12-02T09:42:46.007906Z  INFO ThreadId(16) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:42:46.008074Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.83s | Avg TPS: 2.18
2025-12-02T09:42:46.011849Z  INFO ThreadId(16) Starting upscale job a1645005-c829-4ad5-9b75-91edafc53d3a: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4071.JPG"
2025-12-02T09:42:46.015673Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:46.015931Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:46.016009Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:46.078675Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.0
2025-12-02T09:42:47.831702Z  INFO ThreadId(16) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:42:47.831880Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.82s | Avg TPS: 2.20
2025-12-02T09:42:47.836348Z  INFO ThreadId(16) Starting upscale job 9ca405c5-f81e-44bb-a322-95f15e106921: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4073.JPG"
2025-12-02T09:42:47.840256Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:47.840405Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:47.840490Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:47.916449Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 13.2
2025-12-02T09:42:49.623462Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:42:49.623647Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.78s | Avg TPS: 2.24
2025-12-02T09:42:49.627765Z  INFO ThreadId(16) Starting upscale job 7b49fdcf-c338-4322-b2ad-3ea46635974e: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4107.JPG"
2025-12-02T09:42:49.631162Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:49.631261Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:49.631341Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:49.704897Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 13.6
2025-12-02T09:42:51.400752Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.00s | Total: 0.84s
2025-12-02T09:42:51.400899Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.77s | Avg TPS: 2.26
2025-12-02T09:42:51.404706Z  INFO ThreadId(16) Starting upscale job 68c83c02-4e0b-4f71-af36-75de86202f25: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4126.JPG"
2025-12-02T09:42:51.409530Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:51.409693Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:51.409790Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:51.477104Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 14.9
2025-12-02T09:42:53.205319Z  INFO ThreadId(16) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T09:42:53.205512Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.80s | Avg TPS: 2.23
2025-12-02T09:42:53.209681Z  INFO ThreadId(16) Starting upscale job 8a934f29-f1d4-4c1c-a57a-cae265aaa0cf: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4136.JPG"
2025-12-02T09:42:53.213432Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:53.213520Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:53.213583Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:53.271916Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 17.1
2025-12-02T09:42:54.981058Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:42:54.981214Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.77s | Avg TPS: 2.26
2025-12-02T09:42:54.985153Z  INFO ThreadId(16) Starting upscale job 7dd25a54-c40b-40d1-8789-ad210c8c2baf: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4174.JPG"
2025-12-02T09:42:54.988964Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:54.989085Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:54.989149Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:55.052048Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T09:42:56.814272Z  INFO ThreadId(16) Post-Processing | Encode: 0.86s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:42:56.814433Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.83s | Avg TPS: 2.19
2025-12-02T09:42:56.818465Z  INFO ThreadId(16) Starting upscale job 0d981ed5-02cb-44b7-9128-ea4770170170: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4178.JPG"
2025-12-02T09:42:56.821794Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:56.821897Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:56.822002Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:56.875097Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 18.8
2025-12-02T09:42:58.636508Z  INFO ThreadId(16) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T09:42:58.636701Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.81s | Avg TPS: 2.20
2025-12-02T09:42:58.641093Z  INFO ThreadId(16) Starting upscale job 22a22a94-9b74-4673-8779-58bc0c9a9969: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4180.JPG"
2025-12-02T09:42:58.644601Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:42:58.644790Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:42:58.644922Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:42:58.709162Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.6
2025-12-02T09:43:00.386032Z  INFO ThreadId(16) Post-Processing | Encode: 0.82s | Metadata+Save: 0.00s | Total: 0.82s
2025-12-02T09:43:00.386238Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.74s | Avg TPS: 2.30
2025-12-02T09:43:00.390088Z  INFO ThreadId(16) Starting upscale job 5e19c86c-2d9b-43fc-8bd2-f1e88663e7f1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4184.JPG"
2025-12-02T09:43:00.393760Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:00.393898Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:00.393974Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:00.456368Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.0
2025-12-02T09:43:02.146856Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:43:02.147020Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.75s | Avg TPS: 2.28
2025-12-02T09:43:02.151141Z  INFO ThreadId(16) Starting upscale job 904b5930-a21c-46ae-9f9e-4f6742b2e530: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4190.JPG"
2025-12-02T09:43:02.154565Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:02.154665Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:02.154755Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:02.214581Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.7
2025-12-02T09:43:03.958072Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.02s | Total: 0.85s
2025-12-02T09:43:03.958241Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.80s | Avg TPS: 2.22
2025-12-02T09:43:03.962720Z  INFO ThreadId(16) Starting upscale job 8166f494-372f-4840-8bf1-24bd4fd95553: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4215.JPG"
2025-12-02T09:43:03.967016Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:03.967178Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:03.967266Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:04.027945Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.5
2025-12-02T09:43:05.808916Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.86s
2025-12-02T09:43:05.809066Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.84s | Avg TPS: 2.17
2025-12-02T09:43:05.812457Z  INFO ThreadId(16) Starting upscale job c0646ca6-f5b6-4342-8826-a67ce80a670f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4327.JPG"
2025-12-02T09:43:05.816270Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:05.816382Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:05.816469Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:05.878411Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.1
2025-12-02T09:43:07.622392Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:43:07.622567Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.81s | Avg TPS: 2.21
2025-12-02T09:43:07.627003Z  INFO ThreadId(16) Starting upscale job 81420cc0-ee4a-4096-9397-558fb909d414: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4331.JPG"
2025-12-02T09:43:07.630628Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:07.630731Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:07.630818Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:07.692438Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.2
2025-12-02T09:43:09.451867Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:43:09.452024Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.82s | Avg TPS: 2.20
2025-12-02T09:43:09.455468Z  INFO ThreadId(16) Starting upscale job a6c78edd-bcac-40f0-8fba-d33073e73ea9: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4359.JPG"
2025-12-02T09:43:09.458995Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:09.459123Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:09.459212Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:09.519390Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.6
2025-12-02T09:43:11.270545Z  INFO ThreadId(16) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T09:43:11.270725Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.81s | Avg TPS: 2.21
2025-12-02T09:43:11.275122Z  INFO ThreadId(16) Starting upscale job f4b79711-2deb-4bcb-98ef-26004109d26c: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4385.JPG"
2025-12-02T09:43:11.279765Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:11.279938Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:11.280071Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:11.351803Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 13.9
2025-12-02T09:43:13.114690Z  INFO ThreadId(16) Post-Processing | Encode: 0.87s | Metadata+Save: 0.00s | Total: 0.87s
2025-12-02T09:43:13.114840Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.83s | Avg TPS: 2.18
2025-12-02T09:43:13.119021Z  INFO ThreadId(16) Starting upscale job 7a5937dd-3a6e-401d-bf65-9614933b5e4d: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4407.JPG"
2025-12-02T09:43:13.122529Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:13.122643Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:13.122753Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:13.191906Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 14.5
2025-12-02T09:43:14.903630Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:43:14.903820Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.78s | Avg TPS: 2.25
2025-12-02T09:43:14.907257Z  INFO ThreadId(16) Starting upscale job 4386f90d-4154-41e9-bfe9-8dbabe47d5a6: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4421.JPG"
2025-12-02T09:43:14.910756Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:14.910889Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:14.910986Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:14.970351Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.8
2025-12-02T09:43:16.678557Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:43:16.678706Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.77s | Avg TPS: 2.26
2025-12-02T09:43:16.682123Z  INFO ThreadId(16) Starting upscale job d9ae4849-9b5a-455f-9cb7-cee3414ac254: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4443.JPG"
2025-12-02T09:43:16.685553Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:16.685736Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:16.685856Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:16.745447Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.8
2025-12-02T09:43:18.464412Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:43:18.464575Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.78s | Avg TPS: 2.25
2025-12-02T09:43:18.468557Z  INFO ThreadId(16) Starting upscale job 66b94a93-503b-4c3f-90d5-e35cd39c82b2: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4445.JPG"
2025-12-02T09:43:18.472384Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:18.472529Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:18.472665Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:18.531824Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.9
2025-12-02T09:43:20.288960Z  INFO ThreadId(16) Post-Processing | Encode: 0.87s | Metadata+Save: 0.01s | Total: 0.88s
2025-12-02T09:43:20.289128Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.82s | Avg TPS: 2.20
2025-12-02T09:43:20.292824Z  INFO ThreadId(16) Starting upscale job 20447a41-8bd4-4f5f-b9a8-0f9bcfd4410f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4486.JPG"
2025-12-02T09:43:20.296452Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:20.296551Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:20.296651Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:20.355368Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 17.0
2025-12-02T09:43:22.078947Z  INFO ThreadId(16) Post-Processing | Encode: 0.86s | Metadata+Save: 0.01s | Total: 0.87s
2025-12-02T09:43:22.079121Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.78s | Avg TPS: 2.24
2025-12-02T09:43:22.082977Z  INFO ThreadId(16) Starting upscale job e97044c5-d5ed-4e93-8e7d-3e11723eeebc: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4502.JPG"
2025-12-02T09:43:22.086428Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:22.086596Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:22.086694Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:22.154428Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 14.8
2025-12-02T09:43:23.871580Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.01s | Total: 0.85s
2025-12-02T09:43:23.871766Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.79s | Avg TPS: 2.24
2025-12-02T09:43:23.875715Z  INFO ThreadId(16) Starting upscale job 7c5eff82-5791-4ed7-9564-735e36151b25: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4541.JPG"
2025-12-02T09:43:23.879023Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:23.879157Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:23.879302Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:23.941272Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 16.1
2025-12-02T09:43:25.662242Z  INFO ThreadId(16) Post-Processing | Encode: 0.84s | Metadata+Save: 0.01s | Total: 0.85s
2025-12-02T09:43:25.662423Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.78s | Avg TPS: 2.24
2025-12-02T09:43:25.666326Z  INFO ThreadId(16) Starting upscale job caec6e25-91d5-44ce-95f8-cf382b1cfc32: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4645.JPG"
2025-12-02T09:43:25.669837Z  INFO ThreadId(16) Model Scale: 4x | Target Scale: 4x
2025-12-02T09:43:25.669956Z  INFO ThreadId(16) Model Constraint: Fixed input size 512
2025-12-02T09:43:25.670055Z  INFO ThreadId(16) Scaling Match. Running single pass.
2025-12-02T09:43:25.732789Z  INFO ThreadId(16) Pipeline Active | Batch #1 | TPS: 15.9
2025-12-02T09:43:27.466983Z  INFO ThreadId(16) Post-Processing | Encode: 0.85s | Metadata+Save: 0.00s | Total: 0.85s
2025-12-02T09:43:27.467149Z  INFO ThreadId(16) Upscale Complete | Total Batches: 4 | Duration: 1.80s | Avg TPS: 2.23