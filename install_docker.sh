#! /bin/sh
echo "Creating data directory..."
mkdir -p ./kindly-rss-data/data

echo "Pulling latest docker image..."
docker pull nicoan/kindly-rss-reader

echo "Running the container.."
docker run \
    -d \
    -p 3000:3000 \
    -v "$(pwd)/kindly-rss-data/data:/home/kindlyrss/data" \
    --name kindly-rss-reader \
    nicoan/kindly-rss-reader
