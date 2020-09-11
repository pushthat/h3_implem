use geo_index::{GeoIndex, InMemoryIndexStorage};
use libh3::geo_to_h3;
use libh3::{degs_to_rads, GeoCoord};
mod geo_index;

fn main() {
    let mut in_memory: InMemoryIndexStorage = GeoIndex::new();
    let wellington_verts: Vec<GeoCoord> = vec![
        (174.8009379, -41.2250136),
        (174.7852441, -41.2225049),
        (174.7691999, -41.2225049),
        (174.7535061, -41.2250136),
        (174.7464719, -41.2267323),
        (174.7318140, -41.2316398),
        (174.7188329, -41.2387312),
        (174.7080958, -41.2476959),
        (174.7037477, -41.2521945),
        (174.6957233, -41.2626402),
        (174.6907631, -41.2741098),
        (174.6890838, -41.2861018),
        (174.6890829, -41.2916599),
        (174.6907582, -41.3036500),
        (174.6957152, -41.3151142),
        (174.7037371, -41.3255520),
        (174.7080847, -41.3300466),
        (174.7188218, -41.3390022),
        (174.7318042, -41.3460848),
        (174.7464644, -41.3509854),
        (174.7535002, -41.3527018),
        (174.7691978, -41.3552067),
        (174.7852462, -41.3552067),
        (174.8009438, -41.3527018),
        (174.5384216, -41.5512417),
        (174.6764374, -41.5938781),
        (174.8247528, -41.5933645),
        (175.1454163, -41.6626532),
        (175.5807495, -41.6811176),
        (175.8691406, -41.3386693),
        (176.2879944, -40.8844479),
        (175.1522827, -40.6670979),
        (174.8625183, -40.9902648),
        (174.8653611, -41.2916599),
        (174.8653602, -41.2861018),
        (174.8636809, -41.2741098),
        (174.8587207, -41.2626402),
        (174.8506963, -41.2521945),
        (174.8463482, -41.2476959),
        (174.8356111, -41.2387312),
        (174.8226300, -41.2316398),
        (174.8079721, -41.2267323),
        (174.8009379, -41.2250136),
    ]
    .iter()
    .map(|v| GeoCoord::new(degs_to_rads(v.1), degs_to_rads(v.0)))
    .collect();
    in_memory.insert_zone_from_geo(String::from("wellington_verts"), wellington_verts);
    let geo_cord = GeoCoord::new(degs_to_rads(-41.269837), degs_to_rads(174.777712));
    let key = in_memory
        .get_zone_from_pos(geo_cord)
        .expect("no zone registered");
    println!("{}", key);

    let geo_cord = GeoCoord::new(degs_to_rads(-41.214098), degs_to_rads(175.228362));
    let key = in_memory
        .get_zone_from_pos(geo_cord)
        .expect("no zone registered");
    println!("{}", key);

    let geo_cord = GeoCoord::new(degs_to_rads(48.853970), degs_to_rads(2.373193));
    if !in_memory.get_zone_from_pos(geo_cord).is_some() {
        println!("zenly office is not registered in any zone, let's change that !");
        // have some fun with this wonderfull demo
    }

    let geo_cord = geo_to_h3(
        &GeoCoord::new(degs_to_rads(-41.214098), degs_to_rads(175.228362)),
        10,
    )
    .unwrap();
    let key = in_memory
        .get_zone_from_h3(geo_cord)
        .expect("no zone registered");
    println!("{}", key);

    let geo_cord = geo_to_h3(
        &GeoCoord::new(degs_to_rads(-41.214098), degs_to_rads(175.228362)),
        12,
    )
    .unwrap();
    let key = in_memory
        .get_zone_from_h3(geo_cord)
        .expect("no zone registered");
    println!("{}", key);

    let geo_cord = geo_to_h3(
        &GeoCoord::new(degs_to_rads(-41.225289), degs_to_rads(175.364485)),
        4,
    )
    .unwrap();
    let key = in_memory
        .get_zone_from_h3(geo_cord)
        .expect("no zone registered");
    println!("{}", key);

    let h3_zone = geo_to_h3(
        &GeoCoord::new(degs_to_rads(48.853970), degs_to_rads(2.373193)),
        1,
    )
    .unwrap();
    if !in_memory.get_zone_from_h3(h3_zone).is_some() {
        println!("zenly office is not registered in any zone, let's change that !");
        // have some fun with this wonderfull demo
    }

    let h3_zone = geo_to_h3(
        &GeoCoord::new(degs_to_rads(41.382484), degs_to_rads(2.184997)),
        4,
    )
    .unwrap();
    in_memory.insert_zone_from_h3(String::from("Barcelona"), h3_zone);

    let key = in_memory.get_zone_from_pos(GeoCoord::new(degs_to_rads(41.382484), degs_to_rads(2.184997))).unwrap();
    println!("{}", key);

    let h3_zone = geo_to_h3(
        &GeoCoord::new(degs_to_rads(47.603315), degs_to_rads(-122.333633)),
        7,
    )
    .unwrap();
    in_memory.insert_zone_from_h3(String::from("Seattle"), h3_zone);

    let key = in_memory.get_zone_from_pos(GeoCoord::new(degs_to_rads(47.603315), degs_to_rads(-122.333633))).unwrap();
    println!("{}", key);

    let h3_zone = geo_to_h3(
        &GeoCoord::new(degs_to_rads(-34.599034), degs_to_rads(-58.386375)),
        9,
    )
    .unwrap();
    in_memory.insert_zone_from_h3(String::from("Buenos Aires"), h3_zone);

    let key = in_memory.get_zone_from_pos(GeoCoord::new(degs_to_rads(-34.599034), degs_to_rads(-58.386375))).unwrap();
    println!("{}", key);
}
