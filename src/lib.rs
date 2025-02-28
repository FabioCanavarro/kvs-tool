use core::fmt;
use std::{collections::HashMap, convert::TryInto, error::Error, fs::File, io::{Bytes, Write}, path::{Path, PathBuf}};
extern crate serde_json;
extern crate serde;


pub fn load_file(name: &str) -> Result<File, std::io::Error> {
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(name)
        
}


#[derive(Debug)]
pub enum KvError{
    WriteError,
    ReadError,
}

impl fmt::Display for KvError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self{
            KvError::WriteError => writeln!(f,"Writing has failed!"),
            KvError::ReadError => {Ok(())},
        }
    }
}


impl Error for KvError{}

pub type KvResult<T> = Result<T,crate::KvError>;


#[derive(Debug)]
pub struct KvStore {
    pos: u64,
    path: PathBuf,
}

impl KvStore {
    pub fn new() -> KvStore {
        let _ = File::create("log.txt");
        KvStore {
            pos: 0,
            path: PathBuf::new(),
        }
    }

    pub fn set(&mut self, key: String, val: String) -> KvResult<()>{
        let cmd = Command::set(key, val);
        let mut f = load_file("log.txt").unwrap();
        match f.write(b"{}njjefnjenfjenfjef"){
            Ok(_) => {println!("Writing finish")},
            Err(_) => return Err(KvError::WriteError)
        }
        

        Ok(())
    }

    pub fn get(&self, key: String) -> KvResult<Option<String>> {
        Ok(Some("".to_string()))    
    }

    pub fn remove(&mut self, key: String) -> KvResult<()>{
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf> + AsRef<Path> + Copy) -> KvResult<KvStore>{
        match File::open("log.txt"){
            Ok(_) => {},
            Err(_) => {let _ = File::create("log.txt");}
        }

        Ok(KvStore{path:Into::into(path),pos:0})
        
    }
}

impl Default for KvStore {
    fn default() -> Self {
        KvStore::new()
    }
}

#[derive(Debug)]
enum Command{
    Set{key:String,val:String},
    Get{key:String},
    Remove{key:String,val:String},
}

impl Command{
    fn set(key: String, val: String) -> Command{
        Command::Set { key, val }
    }

    fn get(key: String) -> Command{
        Command::Get { key }
    }

    fn remove(key: String, val: String) -> Command{
        Command::Remove { key, val }
    }

}


































