Running `target\debug\tauri-apprustscale.exe`
2025-12-02T10:10:48.549536Z  INFO ThreadId(01) Logging initialized (Console Only)
2025-12-02T10:10:50.456589Z  INFO ThreadId(16) Loading model from: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx"
2025-12-02T10:10:51.497002Z  INFO ThreadId(16) Model loaded: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx" | Input Type: Float32
2025-12-02T10:10:51.497249Z  INFO ThreadId(16) Detecting scale for: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx"
2025-12-02T10:10:58.312535Z  WARN ThreadId(01) NewEvents emitted without explicit RedrawEventsCleared
2025-12-02T10:10:58.315833Z  WARN ThreadId(01) RedrawEventsCleared emitted without explicit MainEventsCleared
2025-12-02T10:11:02.336606Z  WARN ThreadId(01) NewEvents emitted without explicit RedrawEventsCleared
2025-12-02T10:11:02.341445Z  WARN ThreadId(01) RedrawEventsCleared emitted without explicit MainEventsCleared
2025-12-02T10:11:02.456565Z  INFO ThreadId(14) Starting batch job 2931c2a8-22c0-4d26-b905-8a8f94f599de: 35 files
2025-12-02T10:11:02.456839Z  INFO ThreadId(14) Loading model session: SAFMNv3_x4.onnx
2025-12-02T10:11:02.457150Z  INFO ThreadId(14) Cache Hit: "C:\\Users\\reayu\\OneDrive\\Desktop\\Work Notes\\RustScale\\app\\src-tauri\\models\\SAFMNv3_x4.onnx" 
2025-12-02T10:11:02.457258Z  INFO ThreadId(14) Batch Model Loaded: SAFMNv3_x4.onnx
2025-12-02T10:11:02.457784Z  INFO ThreadId(21) Starting upscale job 66ea504b-bed5-4fa9-b348-8e6f285c0f2f: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3860.JPG"
2025-12-02T10:11:02.463558Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:02.463731Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:02.463840Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:02.871305Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 2.5
2025-12-02T10:11:03.740174Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 1.28s | Avg TPS: 3.13
2025-12-02T10:11:03.743977Z  INFO ThreadId(35) Starting upscale job 2feccd0e-4127-4ee4-9387-a740c1409b60: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3870.JPG"
2025-12-02T10:11:03.752056Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:03.752258Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:03.752375Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:03.821966Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 14.4
2025-12-02T10:11:04.678892Z  INFO ThreadId(21) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T10:11:04.726145Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.97s | Avg TPS: 4.11
2025-12-02T10:11:04.731803Z  INFO ThreadId(35) Starting upscale job 1f0b6303-9624-4354-b596-0adace1ee605: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3880.JPG"
2025-12-02T10:11:04.737940Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:04.738153Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:04.738281Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:04.825160Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 11.5
2025-12-02T10:11:05.742713Z  INFO ThreadId(21) Post-Processing | Encode: 0.98s | Metadata+Save: 0.03s | Total: 1.01s
2025-12-02T10:11:05.749686Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 1.01s | Avg TPS: 3.95
2025-12-02T10:11:05.753377Z  INFO ThreadId(35) Starting upscale job 0f9b3ad9-435b-4d6c-8f81-bf72a64cc396: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3888.JPG"
2025-12-02T10:11:05.758720Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:05.758873Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:05.758969Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:05.834171Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 13.3
2025-12-02T10:11:06.750835Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.99s | Avg TPS: 4.03
2025-12-02T10:11:06.754657Z  INFO ThreadId(39) Starting upscale job 71c173e7-f1a5-40d7-bcf8-d1ec0492c804: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3890.JPG"
2025-12-02T10:11:06.759372Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:06.759519Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:06.759627Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:06.762740Z  INFO ThreadId(21) Post-Processing | Encode: 1.01s | Metadata+Save: 0.00s | Total: 1.01s
2025-12-02T10:11:06.832422Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 13.7
2025-12-02T10:11:07.889445Z  INFO ThreadId(35) Post-Processing | Encode: 1.13s | Metadata+Save: 0.00s | Total: 1.14s
2025-12-02T10:11:07.898703Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 1.14s | Avg TPS: 3.51
2025-12-02T10:11:07.903241Z  INFO ThreadId(35) Starting upscale job c20dace1-b72d-4ed3-bba8-2bf382f9a2c0: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_3950.JPG"
2025-12-02T10:11:07.908384Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:07.908589Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:07.908713Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:08.005040Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 10.4
2025-12-02T10:11:08.936599Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 1.03s | Avg TPS: 3.89
2025-12-02T10:11:08.940693Z  INFO ThreadId(35) Starting upscale job e95994b9-960f-4ccc-8df9-f6bf0bf0f2a4: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4039.JPG"
2025-12-02T10:11:08.947502Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:08.947668Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:08.947804Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:09.002989Z  INFO ThreadId(21) Post-Processing | Encode: 1.10s | Metadata+Save: 0.00s | Total: 1.10s
2025-12-02T10:11:09.037312Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 11.2
2025-12-02T10:11:09.903115Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.96s | Avg TPS: 4.19
2025-12-02T10:11:09.906886Z  INFO ThreadId(35) Starting upscale job b5e95f97-7c44-455a-b371-cf878bf29ada: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4043.JPG"
2025-12-02T10:11:09.914956Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:09.915110Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:09.915217Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:09.917496Z  INFO ThreadId(39) Post-Processing | Encode: 0.98s | Metadata+Save: 0.00s | Total: 0.98s
2025-12-02T10:11:10.004469Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 11.2
2025-12-02T10:11:10.838216Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.33
2025-12-02T10:11:10.841827Z  INFO ThreadId(35) Starting upscale job fd93bcae-491a-412f-b885-4d7ff7bd6805: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4047.JPG"
2025-12-02T10:11:10.849896Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:10.850027Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:10.850140Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:10.861621Z  INFO ThreadId(21) Post-Processing | Encode: 0.95s | Metadata+Save: 0.00s | Total: 0.95s
2025-12-02T10:11:10.925119Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 13.3
2025-12-02T10:11:11.794132Z  INFO ThreadId(39) Post-Processing | Encode: 0.95s | Metadata+Save: 0.00s | Total: 0.95s
2025-12-02T10:11:11.799062Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.95s | Avg TPS: 4.22
2025-12-02T10:11:11.802224Z  INFO ThreadId(39) Starting upscale job e5d582a4-df56-4267-aea4-a27dd75d836c: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4053.JPG"
2025-12-02T10:11:11.807487Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:11.807613Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:11.807709Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:11.878133Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 14.2
2025-12-02T10:11:12.785639Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 0.98s | Avg TPS: 4.09
2025-12-02T10:11:12.789133Z  INFO ThreadId(39) Starting upscale job 27959b02-fd44-4ec3-b16b-edb881cae6be: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4056.JPG"
2025-12-02T10:11:12.789655Z  INFO ThreadId(21) Post-Processing | Encode: 0.99s | Metadata+Save: 0.00s | Total: 0.99s
2025-12-02T10:11:12.794337Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:12.794517Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:12.794635Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:12.876273Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 12.3
2025-12-02T10:11:13.816620Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 1.02s | Avg TPS: 3.91
2025-12-02T10:11:13.820887Z  INFO ThreadId(39) Starting upscale job 51847ca4-3c73-43c5-b307-82f7726b0f4a: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4070.JPG"
2025-12-02T10:11:13.829542Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:13.829669Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:13.829753Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:13.835724Z  INFO ThreadId(35) Post-Processing | Encode: 1.04s | Metadata+Save: 0.00s | Total: 1.05s
2025-12-02T10:11:13.905962Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 13.1
2025-12-02T10:11:14.815743Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 0.99s | Avg TPS: 4.06
2025-12-02T10:11:14.820052Z  INFO ThreadId(39) Starting upscale job 2c6998bc-3f01-48b6-b39b-35681ae38cdb: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4071.JPG"
2025-12-02T10:11:14.820091Z  INFO ThreadId(21) Post-Processing | Encode: 1.00s | Metadata+Save: 0.00s | Total: 1.00s
2025-12-02T10:11:14.824973Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:14.825127Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:14.825245Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:14.898763Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 13.6
2025-12-02T10:11:15.753486Z  INFO ThreadId(35) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T10:11:15.784762Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 0.96s | Avg TPS: 4.17
2025-12-02T10:11:15.788775Z  INFO ThreadId(35) Starting upscale job e715fe6f-37df-4887-ae82-608871805e41: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4073.JPG"
2025-12-02T10:11:15.793369Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:15.793526Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:15.793645Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:15.876583Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 12.1
2025-12-02T10:11:16.739787Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.95s | Avg TPS: 4.23
2025-12-02T10:11:16.743233Z  INFO ThreadId(35) Starting upscale job 94f44b88-9265-4c66-a1e3-b5fbf49557db: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4107.JPG"
2025-12-02T10:11:16.747699Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:16.747841Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:16.747938Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:16.772237Z  INFO ThreadId(21) Post-Processing | Encode: 0.98s | Metadata+Save: 0.00s | Total: 0.98s
2025-12-02T10:11:16.823773Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 13.2
2025-12-02T10:11:17.651130Z  INFO ThreadId(39) Post-Processing | Encode: 0.91s | Metadata+Save: 0.00s | Total: 0.91s
2025-12-02T10:11:17.692207Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.24
2025-12-02T10:11:17.695912Z  INFO ThreadId(39) Starting upscale job 61f46789-34b5-4814-b784-ef08292c9b27: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4126.JPG"
2025-12-02T10:11:17.700563Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:17.700675Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:17.700788Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:17.768520Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 14.8
2025-12-02T10:11:18.630638Z  INFO ThreadId(21) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T10:11:18.641232Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.25
2025-12-02T10:11:18.644174Z  INFO ThreadId(21) Starting upscale job 116ab1fc-dd02-4ca6-8e50-d0b0a2f0cc02: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4136.JPG"
2025-12-02T10:11:18.649186Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:18.649358Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:18.649489Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:18.715061Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 15.3
2025-12-02T10:11:19.554261Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.42
2025-12-02T10:11:19.558236Z  INFO ThreadId(21) Starting upscale job ef4e8ad6-342a-437b-8141-ed5d0f282607: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4174.JPG"
2025-12-02T10:11:19.562764Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:19.562903Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:19.563002Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:19.582072Z  INFO ThreadId(35) Post-Processing | Encode: 0.94s | Metadata+Save: 0.00s | Total: 0.94s
2025-12-02T10:11:19.634645Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 14.0
2025-12-02T10:11:20.490896Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T10:11:20.494811Z  INFO ThreadId(21) Starting upscale job 59f97870-6337-4f0c-8799-afc5bc5248a1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4178.JPG"
2025-12-02T10:11:20.503115Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:20.503258Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:20.503363Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:20.514420Z  INFO ThreadId(39) Post-Processing | Encode: 0.95s | Metadata+Save: 0.00s | Total: 0.96s
2025-12-02T10:11:20.574394Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 14.1
2025-12-02T10:11:21.408738Z  INFO ThreadId(35) Post-Processing | Encode: 0.91s | Metadata+Save: 0.00s | Total: 0.91s
2025-12-02T10:11:21.410463Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.41
2025-12-02T10:11:21.413456Z  INFO ThreadId(35) Starting upscale job 765bf325-1ac2-4850-b755-7bc90d11f3f7: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4180.JPG"
2025-12-02T10:11:21.418315Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:21.418442Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:21.418533Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:21.485063Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 15.0
2025-12-02T10:11:22.324739Z  INFO ThreadId(39) Post-Processing | Encode: 0.89s | Metadata+Save: 0.02s | Total: 0.91s
2025-12-02T10:11:22.329097Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.39
2025-12-02T10:11:22.332057Z  INFO ThreadId(39) Starting upscale job 2ce7415c-01fb-4681-bae3-693fb3544a65: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4184.JPG"
2025-12-02T10:11:22.336754Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:22.337077Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:22.337248Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:22.405188Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 14.7
2025-12-02T10:11:23.240423Z  INFO ThreadId(21) Post-Processing | Encode: 0.91s | Metadata+Save: 0.00s | Total: 0.91s
2025-12-02T10:11:23.299951Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 0.96s | Avg TPS: 4.15
2025-12-02T10:11:23.303782Z  INFO ThreadId(21) Starting upscale job bea224cd-5766-46d1-bf73-6a7615e4a8f1: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4190.JPG"
2025-12-02T10:11:23.308234Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:23.308369Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:23.308478Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:23.377662Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 14.5
2025-12-02T10:11:24.218596Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.40
2025-12-02T10:11:24.223335Z  INFO ThreadId(21) Starting upscale job 8f6ffee1-fa42-4ce5-abf5-7e9a2cc589e8: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4215.JPG"
2025-12-02T10:11:24.231569Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:24.231739Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:24.231838Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:24.262132Z  INFO ThreadId(35) Post-Processing | Encode: 0.96s | Metadata+Save: 0.00s | Total: 0.96s
2025-12-02T10:11:24.305024Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 13.7
2025-12-02T10:11:25.111309Z  INFO ThreadId(39) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:11:25.143048Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.39
2025-12-02T10:11:25.146388Z  INFO ThreadId(39) Starting upscale job 953e15cc-2f8d-4590-a46e-d3e2ff188b15: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4327.JPG"
2025-12-02T10:11:25.151153Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:25.151270Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:25.151412Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:25.219881Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 14.6
2025-12-02T10:11:26.077637Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.32
2025-12-02T10:11:26.080461Z  INFO ThreadId(35) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T10:11:26.082109Z  INFO ThreadId(35) Starting upscale job 4c071088-6131-4ac6-8ee9-e1ba32646e9c: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4331.JPG"
2025-12-02T10:11:26.086627Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:26.086764Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:26.086845Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:26.160403Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 13.6
2025-12-02T10:11:26.989179Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.43
2025-12-02T10:11:26.992790Z  INFO ThreadId(35) Starting upscale job e9afa7f4-bef3-4b78-a343-a675e42ec7a8: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4359.JPG"
2025-12-02T10:11:26.994856Z  INFO ThreadId(21) Post-Processing | Encode: 0.91s | Metadata+Save: 0.00s | Total: 0.91s
2025-12-02T10:11:26.999006Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:26.999152Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:26.999257Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:27.066080Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 15.0
2025-12-02T10:11:27.897442Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.45
2025-12-02T10:11:27.901548Z  INFO ThreadId(35) Starting upscale job f3ee242a-ab7b-4634-aa0d-80e6e9255ca9: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4385.JPG"
2025-12-02T10:11:27.901730Z  INFO ThreadId(39) Post-Processing | Encode: 0.91s | Metadata+Save: 0.00s | Total: 0.91s
2025-12-02T10:11:27.906921Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:27.907105Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:27.907226Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:27.973042Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 15.2
2025-12-02T10:11:28.821893Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.37
2025-12-02T10:11:28.825363Z  INFO ThreadId(35) Starting upscale job 7d1c3c5a-4562-4529-be81-b36d1abd7ce4: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4407.JPG"
2025-12-02T10:11:28.827972Z  INFO ThreadId(21) Post-Processing | Encode: 0.92s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T10:11:28.832116Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:28.832265Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:28.832377Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:28.901721Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 14.4
2025-12-02T10:11:29.742395Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.40
2025-12-02T10:11:29.746069Z  INFO ThreadId(35) Starting upscale job 4e6a64e9-957d-45bd-998c-394a9b16c35b: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4421.JPG"
2025-12-02T10:11:29.750887Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:29.750989Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:29.751094Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:29.753862Z  INFO ThreadId(39) Post-Processing | Encode: 0.93s | Metadata+Save: 0.00s | Total: 0.93s
2025-12-02T10:11:29.820277Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 14.5
2025-12-02T10:11:30.626398Z  INFO ThreadId(21) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T10:11:30.679396Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T10:11:30.683394Z  INFO ThreadId(21) Starting upscale job 9c19879e-2b00-4892-9fdd-b1235c33771b: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4443.JPG"
2025-12-02T10:11:30.689436Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:30.689618Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:30.689691Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:30.779249Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 11.2
2025-12-02T10:11:31.594716Z  INFO ThreadId(39) Post-Processing | Encode: 0.91s | Metadata+Save: 0.00s | Total: 0.91s
2025-12-02T10:11:31.627995Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.94s | Avg TPS: 4.26
2025-12-02T10:11:31.631650Z  INFO ThreadId(39) Starting upscale job 15ab5758-361c-4154-84d8-be58945f1909: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4445.JPG"
2025-12-02T10:11:31.635895Z  INFO ThreadId(39) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:31.636016Z  INFO ThreadId(39) Model Constraint: Fixed input size 512
2025-12-02T10:11:31.636125Z  INFO ThreadId(39) Scaling Match. Running single pass.
2025-12-02T10:11:31.703817Z  INFO ThreadId(39) Pipeline Active | Batch #1 | TPS: 14.8
2025-12-02T10:11:32.580979Z  INFO ThreadId(35) Post-Processing | Encode: 0.94s | Metadata+Save: 0.00s | Total: 0.95s
2025-12-02T10:11:32.586539Z  INFO ThreadId(39) Inference Complete | Total Batches: 4 | Duration: 0.95s | Avg TPS: 4.21
2025-12-02T10:11:32.590385Z  INFO ThreadId(35) Starting upscale job ed2c401a-4fea-4da9-925f-6e6013541909: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4486.JPG"
2025-12-02T10:11:32.595146Z  INFO ThreadId(35) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:32.595297Z  INFO ThreadId(35) Model Constraint: Fixed input size 512
2025-12-02T10:11:32.595384Z  INFO ThreadId(35) Scaling Match. Running single pass.
2025-12-02T10:11:32.663729Z  INFO ThreadId(35) Pipeline Active | Batch #1 | TPS: 14.6
2025-12-02T10:11:33.482531Z  INFO ThreadId(21) Post-Processing | Encode: 0.89s | Metadata+Save: 0.00s | Total: 0.89s
2025-12-02T10:11:33.494170Z  INFO ThreadId(35) Inference Complete | Total Batches: 4 | Duration: 0.90s | Avg TPS: 4.45
2025-12-02T10:11:33.497834Z  INFO ThreadId(21) Starting upscale job 2898d678-6ecd-46ae-8126-f6ded7dd3c68: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4502.JPG"
2025-12-02T10:11:33.503666Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:33.503805Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:33.503887Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:33.570085Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 15.1
2025-12-02T10:11:34.431528Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.93s | Avg TPS: 4.31
2025-12-02T10:11:34.435548Z  INFO ThreadId(21) Starting upscale job 9854e18e-07d6-4842-8eda-678b948f5e8d: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4541.JPG"
2025-12-02T10:11:34.435558Z  INFO ThreadId(39) Post-Processing | Encode: 0.94s | Metadata+Save: 0.00s | Total: 0.94s
2025-12-02T10:11:34.441472Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:34.441630Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:34.441724Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:34.509654Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 14.7
2025-12-02T10:11:35.361805Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.92s | Avg TPS: 4.35
2025-12-02T10:11:35.364977Z  INFO ThreadId(21) Starting upscale job 2ce9b4fc-2a43-4c28-a4cd-e7e5b655b031: "C:\\Users\\reayu\\Downloads\\test batch\\IMG_4645.JPG"
2025-12-02T10:11:35.375757Z  INFO ThreadId(21) Model Scale: 4x | Target Scale: 4x
2025-12-02T10:11:35.375891Z  INFO ThreadId(21) Model Constraint: Fixed input size 512
2025-12-02T10:11:35.375997Z  INFO ThreadId(21) Scaling Match. Running single pass.
2025-12-02T10:11:35.386010Z  INFO ThreadId(35) Post-Processing | Encode: 0.95s | Metadata+Save: 0.00s | Total: 0.95s
2025-12-02T10:11:35.444337Z  INFO ThreadId(21) Pipeline Active | Batch #1 | TPS: 14.6
2025-12-02T10:11:36.243933Z  INFO ThreadId(39) Post-Processing | Encode: 0.88s | Metadata+Save: 0.00s | Total: 0.88s
2025-12-02T10:11:36.286875Z  INFO ThreadId(21) Inference Complete | Total Batches: 4 | Duration: 0.91s | Avg TPS: 4.39
2025-12-02T10:11:37.128911Z  INFO ThreadId(35) Post-Processing | Encode: 0.83s | Metadata+Save: 0.00s | Total: 0.84s