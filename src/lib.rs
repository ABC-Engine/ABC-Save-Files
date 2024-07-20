use std::{fs::create_dir_all, io};

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveFile {
    map: FxHashMap<String, Vec<u8>>,
    org_name: String,
}

impl SaveFile {
    pub fn new(orginization_name: String) -> Self {
        SaveFile {
            map: FxHashMap::default(),
            org_name: orginization_name,
        }
    }

    pub fn set_organization(&mut self, name: String) {
        self.org_name = name;
    }

    pub fn add_component<'a, T>(&mut self, key: String, value: T) -> Result<(), serde_json::Error>
    where
        T: Serialize + Deserialize<'a>,
    {
        let serialized = serde_json::to_vec(&value)?;

        self.map.insert(key, serialized);

        Ok(())
    }

    pub fn get_component<'a, T>(&'a self, key: &str) -> Result<T, serde_json::Error>
    where
        T: Serialize + Deserialize<'a>,
    {
        let serialized = self.map.get(key).unwrap();

        let deserialized: T = serde_json::from_slice(&serialized)?;

        Ok(deserialized)
    }

    pub fn get_save_dir(&self) -> String {
        let data_dir = match dirs::data_dir() {
            Some(path) => {
                // should never be none because it would have to be invalid unicode
                path.to_str()
                    .expect("data dir from 'dirs' crate is invalid")
                    .to_string()
            }
            None => "".to_string(),
        };

        let new_path = data_dir.to_string() + "/" + &self.org_name;

        new_path
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self)?;

        let new_path = self.get_save_dir() + "/" + path;
        // if the new path doesn't exist create it
        create_dir_all(new_path.clone())?;

        std::fs::write(new_path, serialized)?;

        Ok(())
    }

    pub fn load_from_file(&self, path: &str) -> Result<Self, std::io::Error> {
        let new_path = self.get_save_dir() + "/" + path;

        let serialized = std::fs::read_to_string(new_path)?;

        let deserialized: SaveFile = serde_json::from_str(&serialized)?;

        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_save_file() {
        let mut save_file = SaveFile::new("ABC-Save-File-Testing".to_string());

        let key = "player health".to_string();
        let value = 100;

        save_file.add_component(key.clone(), value).unwrap();

        let deserialized: i32 = save_file.get_component(&key).unwrap();

        assert_eq!(value, deserialized);
    }

    #[test]
    fn test_save_file_struct() {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct Player {
            health: i32,
            mana: i32,
        }

        let mut save_file = SaveFile::new("ABC-Save-File-Testing".to_string());

        let key = "player".to_string();
        let value = Player {
            health: 100,
            mana: 50,
        };

        save_file.add_component(key.clone(), value.clone()).unwrap();

        let deserialized: Player = save_file.get_component(&key).unwrap();

        assert_eq!(value.health, deserialized.health);
        assert_eq!(value.mana, deserialized.mana);
    }

    #[test]
    #[should_panic]
    fn test_save_file_mismatched_types() {
        let mut save_file = SaveFile::new("ABC-Save-File-Testing".to_string());

        let bool_key = "boolean value".to_string();
        let bool_value = true;

        save_file
            .add_component(bool_key.clone(), bool_value)
            .unwrap();

        let int_key = "integer value".to_string();
        let int_value = 100;

        save_file.add_component(int_key.clone(), int_value).unwrap();

        // This will panic because the types don't match
        let deserialized_bool: i32 = save_file.get_component(&bool_key).unwrap();
        let deserialized_int: bool = save_file.get_component(&int_key).unwrap();
    }

    #[test]
    fn test_many_values() {
        let mut save_file = SaveFile::new("ABC-Save-File-Testing".to_string());

        for i in 0..10000 {
            let key = format!("key {}", i);
            let value = i;

            save_file.add_component(key.clone(), value).unwrap();

            let deserialized: i32 = save_file.get_component(&key).unwrap();

            assert_eq!(value, deserialized);
        }
    }

    #[test]
    fn test_saving_to_file() {
        let mut save_file = SaveFile::new("ABC-Save-File-Testing".to_string());

        let mut key_value_pairs = vec![];

        let mut rng = rand::thread_rng();
        for i in 0..10000 {
            let key = format!("key {}", i);
            let value = rng.gen_range(0..10000);

            save_file.add_component(key.clone(), value).unwrap();

            key_value_pairs.push((key, value));
        }

        let path = "save_file.json";
        println!("Saving to file: {}", save_file.get_save_dir());
        save_file.save_to_file(path).unwrap();

        let loaded_save_file = SaveFile::new("ABC-Save-File-Testing".to_string())
            .load_from_file(path)
            .unwrap();

        for (key, value) in key_value_pairs {
            let deserialized: i32 = loaded_save_file.get_component(&key).unwrap();

            assert_eq!(value, deserialized);
        }
    }
}
