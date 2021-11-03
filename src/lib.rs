//use acid_store::repo::Commit;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
//use serde_json::Value;
//use std::ops::{Index, IndexMut};
//use acid_store::repo::{OpenOptions, value::ValueRepo, OpenMode};
//use acid_store::store::{DirectoryConfig};
use thiserror::Error;
use std::fmt;
use std::io::{Read, Write};
//use json_value_merge::Merge;
use sonic_serde_object::SonicSerdeObject;
#[derive(Serialize, Eq, PartialEq, Deserialize, Debug, Clone)]
pub struct SonicObject {
    //jsonstr: String,
    pub value: SonicSerdeObject,
    spot: usize
}
pub struct SonicPersistObject {
    pub tree: SonicSerdeObject,
    pub filepath: PathBuf,
}
#[derive(Debug, Error)]
pub enum SonicObjectError {
    KeyError(String),
    IndexError(String),
}
impl fmt::Display for SonicObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IndexError(z) => {
                write!(f, "{}", z)
            },
            Self::KeyError(z) => {
                write!(f, "{}", z)
            }
        }
    }
}

impl SonicObject {
    pub fn new(value: impl Into<SonicSerdeObject>) -> Self {
        //let jsonstring = serde_json::to_string(&value).unwrap();
        //let v: Value = serde_json::from_str(jsonstring.as_str()).unwrap();
        let spot: usize = 0;
        ////println!("snew value is '{:?}'", v);
        //let v = serde_json::to_value(value).unwrap();
        let v = value.into();
        Self {
            //jsonstr: jsonstring,
            value: v,
            spot: spot,
        }
    }
    pub fn collectvec(&self) -> Vec<Self> {
        let x = self.clone();
        x.collect::<Vec<Self>>()
    }
    pub fn get(&self, key: impl Into<SonicSerdeObject>) -> Result<SonicObject, SonicObjectError> {
        ////println!("{}", self.value);
        //println!("{:?}", self.value);
        //match self.value.get(serde_json::to_string(&key).unwrap()) {
        //let mut keyval = serde_json::to_value(key).unwrap().to_string();
        let keyclone = key.into();
        match self.value.as_map().unwrap().get(&keyclone.clone()) {
            Some(a) => {
                return Ok(SonicObject::new(a.clone()));
            },
            None => {
                return Err(SonicObjectError::KeyError(format!("No such key {:?}", keyclone.clone())));
            }
        } 
        //SonicObject::new(.unwrap()).unwrap().to_owned())
    }
    pub fn contains(&self, key: impl Into<SonicSerdeObject>) -> bool {
        //let svalue = self.value.as_object().unwrap();
        //svalue.contains_key(&serde_json::to_string(&key).unwrap())
        self.value.as_map().unwrap().contains_key(&key.into())
    }
    pub fn keys(&self) -> Vec<SonicSerdeObject> {
        //let svalue = self.value.as_object().unwrap();
        //svalue.keys().collect::<Vec<&String>>().into_iter().map(|y| serde_json::from_str::<Value>(y.to_string().as_str()).unwrap()).collect()
        //x.into_iter().map(|y| y.to_string()).collect()
        self.value.as_map().unwrap().keys().collect::<Vec<&SonicSerdeObject>>().into_iter().map(|z| z.clone()).collect()

    }
    pub fn insert(&mut self, key: impl Into<SonicSerdeObject>, value: impl Into<SonicSerdeObject>) -> () {
        //let svalue = self.value.as_object_mut().unwrap();
        //println!("svalue is now '{:?}'", svalue);
        ////println!("svalue = '{:?}'", svalue);
        //let sobject = serde_json::to_string(&value).unwrap();
        //let sobject = serde_json::to_value(value).unwrap();
        ////println!("sobject = '{:?}'", sobject);
        //svalue.insert(serde_json::to_string(&key).unwrap(), serde_json::from_str(&sobject).unwrap());
        //self.value.insert(serde_json::to_value(key).unwrap().to_string(), sobject);
        //svalue.remove("value");        
        ////println!("svalue = '{:?}'", svalue);
        //let jsonstring = serde_json::to_string(&svalue).unwrap();
        //let v: Value = serde_json::from_str(&jsonstring).unwrap();        
        
        //self.jsonstr = jsonstring;
        //self.value = v;
        //self.value = serde_json::to_value(svalue).unwrap();
        let mut val = self.value.as_map().unwrap();
        val.insert(key.into(), value.into());
        self.value = SonicSerdeObject::Map(val);
    }
    pub fn replace_index_with(&mut self, index: usize, value: impl Into<SonicSerdeObject>) {
        let mut vvalue = self.value.as_vec().unwrap();
        vvalue.remove(index);
        vvalue.insert(index, value.into());
    }
    pub fn push(&mut self, value: impl Into<SonicSerdeObject>) -> () {
        //if !self.value.is_vec() {
            //println!("Not array {:?}", self.value);
        //}
        let mut svalue = self.value.as_vec().unwrap();
        ////println!("svalue = '{:?}'", svalue);
        //let sobject = serde_json::to_string(&value).unwrap();
        //let sobj = serde_json::to_value(value).unwrap();
        ////println!("sobject = '{:?}'", sobject);
        //svalue.push(serde_json::from_str(&sobject).unwrap());
        svalue.push(value.into());
        //svalue.remove("value");        
        ////println!("svalue = '{:?}'", svalue);
        //let jsonstring = serde_json::to_string(&svalue).unwrap();
        //let v: Value = serde_json::from_str(&jsonstring).unwrap();        
        //self.jsonstr = jsonstring;
        //self.value = v;
        self.value = SonicSerdeObject::Vec(svalue);
    }
    pub fn remove(&mut self, key: impl Into<SonicSerdeObject>) -> () {
        let mut svalue = self.value.as_map().unwrap();
        ////println!("svalue = '{:?}'", svalue);
        //let sobject = serde_json::to_string(&value).unwrap();
        ////println!("sobject = '{:?}'", sobject);
        //svalue.insert(key.to_string(), serde_json::from_str(&sobject).unwrap());
        //svalue.remove(&serde_json::to_string(&key).unwrap());        
        svalue.remove(&key.into());
        ////println!("svalue = '{:?}'", svalue);
        //let jsonstring = serde_json::to_string(&svalue).unwrap();
        //let v: Value = serde_json::from_str(&jsonstring).unwrap();        
        //self.jsonstr = jsonstring;
        //self.value = v;
        //self.value = serde_json::to_value(svalue).unwrap();
        self.value = SonicSerdeObject::Map(svalue);
    }
    pub fn getindex(&self, index: usize) -> Result<SonicObject, SonicObjectError> {
        match self.value.as_vec().unwrap().get(index) {
            Some(a) => {
                return Ok(SonicObject::new(a.clone()))
            },
            None => {
                return Err(SonicObjectError::IndexError("Index out of range".to_string()));
            }
        }
        //SonicObject::new(self.value[index].to_owned())
    }
    pub fn getindexvalue(&self, index: usize) -> Result<SonicSerdeObject, SonicObjectError> {
        //println!("not a vec {:?}", self.value);
        match self.value.as_vec().unwrap().get(index) {
            Some(a) => {
                return Ok(a.clone());
            },
            None => {
                return Err(SonicObjectError::IndexError("Index out of range".to_string()))
            }
        }
        //self.value[index].to_owned()
    }
    pub fn removeindex(&mut self, index: usize) -> () {
        let mut svalue = self.value.as_vec().unwrap();
        ////println!("svalue = '{:?}'", svalue);
        //let sobject = serde_json::to_string(&value).unwrap();
        ////println!("sobject = '{:?}'", sobject);
        //svalue.insert(key.to_string(), serde_json::from_str(&sobject).unwrap());
        svalue.remove(index);        
        ////println!("svalue = '{:?}'", svalue);
        //let jsonstring = serde_json::to_string(&svalue).unwrap();
        //let v: Value = serde_json::from_str(&jsonstring).unwrap();        
        //self.jsonstr = jsonstring;
        //self.value = v;
        self.value = SonicSerdeObject::Vec(svalue);
    }
    pub fn getvalue(&mut self, key: impl Into<SonicSerdeObject>) -> Result<SonicSerdeObject, SonicObjectError> {
        //match self.value.get(serde_json::to_string(&key).unwrap()) {
        //let mut keyval = serde_json::to_value(key).unwrap().to_string();
        let keyclone = key.into();
        match self.value.as_map().unwrap().get(&keyclone.clone()) {
            Some(a) => {
                return Ok(a.clone());
            },
            None => {
                return Err(SonicObjectError::KeyError(format!("No such key {:?}", keyclone.clone())));
            }
        }
        //self.value.get(serde_json::to_string(&key).unwrap()).unwrap().to_owned()
    }
}
/*
impl Index<&'_ str> for SonicObject {
    type Output = Value;
    fn index(&self, s: &str) -> &Value {
        self.value.get(s.to_string()).unwrap()
        //x
    }
}
impl IndexMut<&'_ impl Into<SonicSerdeObject>> for SonicObject {
    fn index_mut(&mut self, s: &str) -> &mut Value {
        self.value.get_mut(s.to_string()).unwrap()
    }
}

impl Index<&'_ usize> for SonicObject {
    type Output = SonicSerdeObject;
    fn index(&self, s: &usize) -> &SonicSerdeObject {
        let x = s.clone();
        self.getindexvalue(x).unwrap().as_ref()
        //x
    }
}
impl IndexMut<&'_ usize> for SonicObject {
    fn index_mut(&mut self, s: &usize) -> &mut SonicSerdeObject {
        self.value.as_vec().unwrap().get_mut(*s).unwrap()
    }
}
impl AsMut<SonicObject> for SonicObject {
    fn as_mut(&mut self) -> &mut SonicObject {
        self
    }
}
*/
impl Iterator for SonicObject {
    type Item = SonicObject;
    fn next(&mut self) -> Option<SonicObject> {
        if self.value.is_vec() {
            let val = self.value.as_vec().unwrap();
            if self.spot == val.len() {
                return None
            } else {
                self.spot = self.spot + 1;
                return Some(SonicObject::new(val[self.spot - 1].clone()))
            }            
        } else {
            None
        }
    }
}

impl SonicPersistObject {
    pub fn new(filep: impl Into<PathBuf>) -> Self {
        let filepath = filep.into();
        let tree: SonicSerdeObject;
        if filepath.exists() {
            let mut file_obj = std::fs::File::open(filepath.clone()).unwrap();
            let mut buf: Vec<u8> = Vec::new();
            file_obj.read_to_end(&mut buf).unwrap();
            tree = rmp_serde::decode::from_slice(&buf).unwrap()
        } else {
            tree = SonicSerdeObject::new_map();
        }
        //let tree = OpenOptions::new().mode(OpenMode::Create).open(&DirectoryConfig{ path: filepath }).unwrap();//sled::open(&filepath).unwrap();
        Self {
            tree: tree,
            filepath: filepath,
        }
    }
    pub fn contains(&self, key: impl Into<String>) -> bool {
        let key_string = key.into();
        self.tree.as_map().unwrap().contains_key(&key_string.into())
    }
    pub fn get(&self, key: impl Into<String>) -> SonicObject {
        let key_string = key.into();
        let u8_vec: Vec<u8> = self.tree.as_map().unwrap().get(&key_string.into()).unwrap().clone().as_vecu8().unwrap();
        //let p: Value = serde_json::from_str(String::from_utf8(self.tree.get(key).unwrap().unwrap().as_ref().to_vec()).unwrap().as_str()).unwrap();
        //let p: SonicSerdeObject = serde_json::from_str(jsonstring.as_str()).unwrap();
        ////println!("p is '{:?}'", p);
        let p: SonicSerdeObject = rmp_serde::decode::from_slice(&u8_vec).unwrap();
        SonicObject::new(p)
    }
    pub fn insert(&mut self, key: impl Into<String>, val: impl Into<SonicSerdeObject>) -> () {
        //let mut sobj = SonicObject::new(value);
        let value = val.into();
        let key_string = key.into();
        let new_vec = rmp_serde::encode::to_vec(&value).unwrap();
        //rmp_serde::encode::to_vec_named(&value.into()); 
        //self.tree.insert(key.to_string(), &serde_json::to_string(&value.into()).unwrap()).unwrap();
        self.tree.insert(key_string, SonicSerdeObject::VecU8(new_vec));
        self.flush();
        //self.tree.().unwrap();
    }
    pub fn flush(&mut self) -> () {
        if self.filepath.exists() {
            std::fs::remove_file(self.filepath.clone()).unwrap();
        }
        let mut file_obj = std::fs::File::create(self.filepath.clone()).unwrap();
        file_obj.write_all(&rmp_serde::encode::to_vec(&self.tree).unwrap()).unwrap();
    }
}

/*
impl SonicPersistObject {
    pub fn new(filepath: PathBuf) -> Self {
        let tree = OpenOptions::new().mode(OpenMode::Create).open(&DirectoryConfig{ path: filepath }).unwrap();//sled::open(&filepath).unwrap();
        Self {
            tree: tree,
        }
    }
    pub fn contains(&self, key: &str) -> bool {
        self.tree.contains(key)
    }
    pub fn get(&self, key: &str) -> SonicObject {
        let u8_vec: Vec<u8> = self.tree.get(&key.to_string()).unwrap();
        //let p: Value = serde_json::from_str(String::from_utf8(self.tree.get(key).unwrap().unwrap().as_ref().to_vec()).unwrap().as_str()).unwrap();
        //let p: SonicSerdeObject = serde_json::from_str(jsonstring.as_str()).unwrap();
        ////println!("p is '{:?}'", p);
        let p: SonicSerdeObject = rmp_serde::decode::from_slice(&u8_vec).unwrap();
        SonicObject::new(p)
    }
    pub fn insert(&mut self, key: &str, value: impl Into<SonicSerdeObject>) -> () {
        //let mut sobj = SonicObject::new(value);
        let new_vec = rmp_serde::encode::to_vec(&value.into()).unwrap();
        //rmp_serde::encode::to_vec_named(&value.into()); 
        //self.tree.insert(key.to_string(), &serde_json::to_string(&value.into()).unwrap()).unwrap();
        self.tree.insert(key.to_string(), &new_vec).unwrap();
        self.tree.commit().unwrap();
    }
    pub fn flush(&mut self) -> () {
        self.tree.commit().unwrap();
    }
}
*/
