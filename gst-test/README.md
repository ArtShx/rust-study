`
# build
docker build -t gst-test:latest .
# run
docker run --rm -it --entrypoint bash --mount type=bind,src=$PWD,dst=/app/ gst-test:latest

### Run 
RTSP server and run gstreamer container from ssh
`docker compose run --service-ports gstreamer bash`

Start RTSP streaming
`ffmpeg -re -stream_loop -1 -i $video -c copy -f rtsp rtsp://172.22.0.2:8554/mystream`
`