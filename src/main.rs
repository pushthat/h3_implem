use geo_index::{GeoIndex, InMemoryIndexStorage};
use grpc_map::map_server::{Map, MapServer};
use grpc_map::{
    GetReply, GetZoneFromGeoRequest, GetZoneFromH3Request, InsertReply, InsertZoneFromGeoRequest,
    InsertZoneFromH3Request,
};

use libh3::{degs_to_rads, GeoCoord};
use std::sync;
use std::sync::Mutex;
use tokio::runtime::Runtime;
use tonic::{transport::Server, Response};

pub mod grpc_map {
    tonic::include_proto!("grpcmap");
}

mod geo_index;

#[derive(Debug)]
pub struct MapInMemory {
    in_memory_mutex: sync::Mutex<InMemoryIndexStorage>,
}

impl Default for MapInMemory {
    fn default() -> Self {
        let in_memory = InMemoryIndexStorage::new();

        let map_in_memory = Self {
            in_memory_mutex: Mutex::new(in_memory),
        };

        return map_in_memory;
    }
}

// trait available in target/debug/take-home-2../out/grpcmap.rs
#[tonic::async_trait]
impl Map for MapInMemory {
    async fn insert_zone_from_geo(
        &self,
        request: tonic::Request<InsertZoneFromGeoRequest>,
    ) -> Result<tonic::Response<InsertReply>, tonic::Status> {
        let request_content = request.into_inner();
        let zone_name = request_content.zone_name;
        let geo_to_insert = request_content
            .geolocations
            .iter()
            .map(|v| GeoCoord::new(degs_to_rads(v.lat as f64), degs_to_rads(v.lon as f64)))
            .collect();

        let resp = self
            .in_memory_mutex
            .lock()
            .unwrap()
            .insert_zone_from_geo(zone_name, geo_to_insert);

        let reply = InsertReply { ok: resp };
        Ok(Response::new(reply))
    }

    async fn insert_zone_from_h3(
        &self,
        request: tonic::Request<InsertZoneFromH3Request>,
    ) -> Result<tonic::Response<InsertReply>, tonic::Status> {
        let request_content = request.into_inner();
        let zone_name = request_content.zone_name;
        let h3_index_to_insert = request_content.h3_index;

        let resp = self
            .in_memory_mutex
            .lock()
            .unwrap()
            .insert_zone_from_h3(zone_name, h3_index_to_insert as u64);

        let reply = InsertReply { ok: resp };
        Ok(Response::new(reply))
    }

    async fn get_zone_from_geo(
        &self,
        request: tonic::Request<GetZoneFromGeoRequest>,
    ) -> Result<tonic::Response<GetReply>, tonic::Status> {
        let request_content = request.into_inner();
        let mut reply = GetReply {
            zone_name: String::from(""),
            found: false,
        };
        if request_content.geolocation.is_some() {
            let geo_to_find = GeoCoord::new(
                degs_to_rads(request_content.geolocation.clone().unwrap().lat as f64),
                degs_to_rads(request_content.geolocation.unwrap().lon as f64),
            );

            let resp = self
                .in_memory_mutex
                .lock()
                .unwrap()
                .get_zone_from_pos(geo_to_find);

            if resp.is_some() {
                reply = GetReply {
                    zone_name: resp.unwrap(),
                    found: true,
                };
            }
        }
        Ok(Response::new(reply))
    }

    async fn get_zone_from_h3(
        &self,
        request: tonic::Request<GetZoneFromH3Request>,
    ) -> Result<tonic::Response<GetReply>, tonic::Status> {
        let request_content = request.into_inner();
        let mut reply = GetReply {
            zone_name: String::from(""),
            found: false,
        };

        let resp = self
            .in_memory_mutex
            .lock()
            .unwrap()
            .get_zone_from_h3(request_content.h3_index as u64);
        if resp.is_some() {
            reply = GetReply {
                zone_name: resp.unwrap(),
                found: true,
            };
        }
        Ok(Response::new(reply))
    }
}

fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let map_in_memory = MapInMemory::default();

    let mut rt = Runtime::new().expect("failed to obtain a new RunTime object");
    let server_future = Server::builder()
        .add_service(MapServer::new(map_in_memory))
        .serve(addr);
    rt.block_on(server_future)
        .expect("failed to successfully run the future on RunTime");
}
