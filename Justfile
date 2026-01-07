deploy:
  cd app && npm run build
  aws s3 cp app/dist/index.html s3://lexi.fyi/index.html --cache-control "max-age=30"
  aws s3 cp --recursive app/dist/assets s3://lexi.fyi/assets/ --cache-control "max-age=31536000"
  