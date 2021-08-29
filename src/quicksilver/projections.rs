use dashmap::DashMap;
use uuid::Uuid;
use std::collections::hash_map::RandomState;
use dashmap::mapref::one::Ref;

lazy_static! {
    pub static ref ID_TO_NAME:DashMap<Uuid, String> = DashMap::new();
}

pub fn add_id_name_mapping(id:Uuid, name: String) {
    ID_TO_NAME.insert(id, name);
}

pub fn id_to_name(id: &Uuid) -> Ref<Uuid, String> {
    let name = ID_TO_NAME.get(&id);
    let asd: Ref<Uuid, String, RandomState> = name.unwrap();
    asd
}
