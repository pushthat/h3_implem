use geo::algorithm::contains::Contains;
use geo::{Coordinate, LineString, Polygon};
use libh3::geo_to_h3;
use libh3::h3_get_resolution;
use libh3::h3_to_children;
use libh3::h3_to_geo_boundary;
use libh3::h3_to_parent;
use libh3::rads_to_degs;
use libh3::H3Index;
use libh3::{polyfill, GeoCoord};
use std::collections::HashMap;

pub trait GeoIndex {
    fn new() -> Self;
    fn insert_zone_from_geo(&mut self, zone_name: String, geolocations: Vec<GeoCoord>) -> bool;
    fn insert_zone_from_h3(&mut self, zone_name: String, h3_index: H3Index) -> bool;
    fn get_zone_from_h3(&mut self, h3_index: H3Index) -> Option<String>;
    fn get_zone_from_pos(&mut self, geolocations: GeoCoord) -> Option<String>;
}

pub struct InMemoryIndexStorage {
    map_store: HashMap<H3Index, String>,
}

impl InMemoryIndexStorage {
    fn find_from_bigger_res_zones(&mut self, h3_index: H3Index) -> Option<String> {
        let h3_childrens = h3_to_children(h3_index, ResolutionDetails::BigResolution as u8);
        for child in &h3_childrens {
            if !self.map_store.contains_key(&child) {
                return None;
            }
        }
        let return_val = self.map_store.get(&h3_childrens[0]).unwrap();
        return Some(return_val.clone());
    }

    fn insert_from_bigger_res_zones(&mut self, h3_index: H3Index, zone_name: String) -> bool {
        let h3_childrens = h3_to_children(h3_index, ResolutionDetails::BigResolution as u8);
        for child in h3_childrens {
            self.map_store.insert(child, zone_name.clone());
        }
        return true;
    }

    fn insert_from_smaller_res_zones(&mut self, h3_index: H3Index, zone_name: String) -> bool {
        let h3_parent = h3_to_parent(h3_index, ResolutionDetails::SmallResolution as u8);
        self.map_store.insert(h3_parent, zone_name.clone());
        return true;
    }

    fn find_from_smaller_res_zones(&mut self, h3_index: H3Index) -> Option<String> {
        let h3_parent = h3_to_parent(h3_index, ResolutionDetails::BigResolution as u8);
        if self.map_store.contains_key(&h3_parent) {
            let return_val = self.map_store.get(&h3_parent).unwrap();
            return Some(return_val.clone());
        }

        let h3_parent = h3_to_parent(h3_index, ResolutionDetails::MediumResolution as u8);
        if self.map_store.contains_key(&h3_parent) {
            let return_val = self.map_store.get(&h3_parent).unwrap();
            return Some(return_val.clone());
        }

        let h3_parent = h3_to_parent(h3_index, ResolutionDetails::SmallResolution as u8);
        if self.map_store.contains_key(&h3_parent) {
            let return_val = self.map_store.get(&h3_parent).unwrap();
            return Some(return_val.clone());
        }
        return None;
    }

    fn get_registered(&mut self, h3_index: H3Index) -> Option<String> {
        if self.map_store.contains_key(&h3_index) {
            let return_val = self.map_store.get(&h3_index).unwrap();
            return Some(return_val.clone());
        }
        return None;
    }
}

fn create_polygon_from_geocord(geolocations: &Vec<GeoCoord>) -> geo::Polygon<f64> {
    let mut line_string_vect = Vec::new();

    for eval in geolocations {
        let coor = Coordinate {
            x: rads_to_degs(eval.lon),
            y: rads_to_degs(eval.lat),
        };
        line_string_vect.push(coor)
    }
    let line_string = LineString(line_string_vect);

    let v = Vec::new();
    let poly = Polygon::new(line_string, v.clone());
    return poly;
}

fn get_childrens_from_h3_index(h3_index: &H3Index, resolution: u8) -> Vec<u64> {
    let childs = h3_to_children(*h3_index, resolution);
    return childs;
}

fn add_h3_vec_to_map(mut map: HashMap<H3Index, bool>, h3_vec: &Vec<u64>) -> HashMap<H3Index, bool> {
    for elem in h3_vec {
        map.insert(*elem, true);
    }
    return map;
}

// resolution details level
enum ResolutionDetails {
    BigResolution = 5,
    MediumResolution = 6,
    SmallResolution = 7,
}

impl GeoIndex for InMemoryIndexStorage {
    fn new() -> Self {
        let in_memory_index_storage = Self {
            map_store: HashMap::new(),
        };
        return in_memory_index_storage;
    }

    // insert a zone from multiple geolocation
    // you can remove the comments to have a better visibility on the multiple layers
    fn insert_zone_from_geo(&mut self, zone_name: String, geolocations: Vec<GeoCoord>) -> bool {
        let orignal_polygon = create_polygon_from_geocord(&geolocations);
        let vec_poly = &vec![geolocations];
        let big_resolution_h3_poly = polyfill(vec_poly, ResolutionDetails::BigResolution as u8);
        let mut already_added_medium: HashMap<H3Index, bool> = HashMap::new();
        let mut already_added_small: HashMap<H3Index, bool> = HashMap::new();
        let mut small_childs: Vec<u64>;
        let mut medium_childs: Vec<u64>;

        // println!("BIG ENTRY with resolution : {}", ResolutionDetails::BigResolution as u8);
        for x in &big_resolution_h3_poly {
            let bondaries = h3_to_geo_boundary(*x);
            let big_sized_poly = create_polygon_from_geocord(&bondaries);

            if orignal_polygon.contains(&big_sized_poly) {
                medium_childs =
                    get_childrens_from_h3_index(x, ResolutionDetails::MediumResolution as u8);
                small_childs =
                    get_childrens_from_h3_index(x, ResolutionDetails::SmallResolution as u8);

                // remove this debug loop
                // let barss = h3_to_geo_boundary(*x);
                // for eval in &barss {
                //     println!("{},{}", rads_to_degs(eval.lon), rads_to_degs(eval.lat));
                // }

                already_added_medium = add_h3_vec_to_map(already_added_medium, &medium_childs);
                already_added_small = add_h3_vec_to_map(already_added_small, &small_childs);
                self.map_store.insert(*x, zone_name.clone());
            }
        }
        // println!("------------------------------------");
        // println!("------------------------------------");
        // println!("MEDIUM ENTRY with resolution : {}", ResolutionDetails::MediumResolution as u8);
        let medium_resolution_h3_poly =
            polyfill(vec_poly, ResolutionDetails::MediumResolution as u8);
        for x in &medium_resolution_h3_poly {
            let bondaries = h3_to_geo_boundary(*x);
            let big_sized_poly = create_polygon_from_geocord(&bondaries);
            if orignal_polygon.contains(&big_sized_poly) && !already_added_medium.contains_key(x) {
                small_childs =
                    get_childrens_from_h3_index(x, ResolutionDetails::SmallResolution as u8);

                // remove this debug loop
                // let barss = h3_to_geo_boundary(*x);
                // for eval in &barss {
                //     println!("{},{}", rads_to_degs(eval.lon), rads_to_degs(eval.lat));
                // }

                already_added_small = add_h3_vec_to_map(already_added_small, &small_childs);
                self.map_store.insert(*x, zone_name.clone());
            }
        }

        // println!("------------------------------------");
        // println!("------------------------------------");
        // println!("SMALL ENTRY with resolution : {}", ResolutionDetails::SmallResolution as u8);
        let small_resolution_h3_poly = polyfill(vec_poly, ResolutionDetails::SmallResolution as u8);
        for x in &small_resolution_h3_poly {
            let bondaries = h3_to_geo_boundary(*x);
            let big_sized_poly = create_polygon_from_geocord(&bondaries);
            if orignal_polygon.contains(&big_sized_poly) && !already_added_small.contains_key(x) {
                // remove this debug loop
                // let barss = h3_to_geo_boundary(*x);
                // for eval in &barss {
                //     println!("{},{}", rads_to_degs(eval.lon), rads_to_degs(eval.lat));
                // }

                self.map_store.insert(*x, zone_name.clone());
            }
        }
        return true;
    }

    fn insert_zone_from_h3(&mut self, zone_name: String, h3_index: H3Index) -> bool {
        let res = h3_get_resolution(h3_index);
        if res < ResolutionDetails::BigResolution as u8 {
            return self.insert_from_bigger_res_zones(h3_index, zone_name);
        } else if res > ResolutionDetails::SmallResolution as u8 {
            return self.insert_from_smaller_res_zones(h3_index, zone_name);
        } else {
            self.map_store.insert(h3_index, zone_name.clone());   
        }

        return true;
    }

    fn get_zone_from_h3(&mut self, h3_index: H3Index) -> Option<String> {
        let res = h3_get_resolution(h3_index);
        if res == ResolutionDetails::BigResolution as u8
            || res == ResolutionDetails::MediumResolution as u8
            || res == ResolutionDetails::SmallResolution as u8
        {
            return self.get_registered(h3_index);
        } else {
            if res < ResolutionDetails::BigResolution as u8 {
                return self.find_from_bigger_res_zones(h3_index);
            } else if res > ResolutionDetails::SmallResolution as u8 {
                return self.find_from_smaller_res_zones(h3_index);
            }
        }
        return None;
    }

    fn get_zone_from_pos(&mut self, geolocations: GeoCoord) -> Option<String> {
        let big_index = geo_to_h3(&geolocations, ResolutionDetails::BigResolution as u8).unwrap();
        if self.map_store.contains_key(&big_index) {
            let return_val = self.map_store.get(&big_index).unwrap();
            return Some(return_val.clone());
        }

        let medium_index =
            geo_to_h3(&geolocations, ResolutionDetails::MediumResolution as u8).unwrap();
        if self.map_store.contains_key(&medium_index) {
            let return_val = self.map_store.get(&medium_index).unwrap();
            return Some(return_val.clone());
        }

        let small_index =
            geo_to_h3(&geolocations, ResolutionDetails::SmallResolution as u8).unwrap();
        if self.map_store.contains_key(&small_index) {
            let return_val = self.map_store.get(&small_index).unwrap();
            return Some(return_val.clone());
        }
        return None;
    }
}
