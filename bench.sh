ghz --insecure \
  --proto ./proto/grpcmap.proto \
  --call grpcmap.Map.GetZoneFromGeo \
  -d '{"geolocation": {"lon": 2.3294449, "lat": 48.8629079} }' \
  -n 200000 \
  -c 100 \
  localhost:50051
