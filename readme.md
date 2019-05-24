# Beat / Onset detection utility
Represents source audiofile as PCM and Beat data

## Available binaries

* `audio_info <audio.mp3>` shows PCM and meta data briefly on terminal
* `pcm_gnu <audio.mp3>` shows PCM data on the GNUPlot
* `pcm_json <audio.mp3>` shows PCM data as JSON
* `fft_gnu <audio.mp3>` shows FFT data on the GNUPlot
* `od_gnu <audio.mp3>` shows peak data on the GNUPlot
* `od_daemon <127.0.0.1:8001>` starts daemon with json rpc API
    * methods
        * `beats` shows beats array
        * `pcm` shows pcm array
        * `fft` shows fft array
    * params:
        * `name` path to the audio
