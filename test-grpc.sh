grpcurl -plaintext -protoset ./proto/grpcmap.protoset -d '{"zone_name": "Paris 1er", "geolocations": [
    {"lon": 2.3188019, "lat": 48.8646583}, 
    {"lon": 2.3207760, "lat": 48.8671991}, 
    {"lon": 2.3407745, "lat": 48.8608751}, 
    {"lon": 2.3401737, "lat": 48.8587858}, 
    {"lon": 2.3188019, "lat": 48.8646583}]}' localhost:50051 grpcmap.Map.InsertZoneFromGeo
grpcurl -plaintext -protoset ./proto/grpcmap.protoset -d '{"geolocation": {"lon": 2.3294449, "lat": 48.8629079} }' localhost:50051 grpcmap.Map.GetZoneFromGeo

grpcurl -plaintext -protoset ./proto/grpcmap.protoset -d '{"h3_index": 578536630256664575, "zone_name": "1/7 of the world"}' localhost:50051 grpcmap.Map.InsertZoneFromH3
grpcurl -plaintext -protoset ./proto/grpcmap.protoset -d '{"h3_index": 578536630256664575}' localhost:50051 grpcmap.Map.GetZoneFromH3
