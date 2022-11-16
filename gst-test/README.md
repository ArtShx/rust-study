`
# build
docker build -t gst-test:latest .
# run
docker run --rm -it --entrypoint bash --mount type=bind,src=$PWD,dst=/app/ gst-test:latest
`