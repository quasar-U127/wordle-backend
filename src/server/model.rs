use crate::server::session;
use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};
pub struct Model {
    data_path: PathBuf,
}

impl Model {
    pub fn new(data_path: &Path) -> Model {
        return Model {
            data_path: data_path.to_path_buf(),
        };
    }

    pub fn get_file_path(&self, id: session::SessionId) -> PathBuf {
        let mut child_path = self.data_path.clone();
        child_path.push(id.to_string() + ".wdl");
        return child_path;
    }

    pub fn new_id(&self) -> session::SessionId {
        return session::SessionId::new(rand::random());
    }

    pub fn store_session(&self, sess: session::Session) -> Result<(), std::io::Error> {
        println!("{}", self.get_file_path(sess.id()).to_str().unwrap());
        let mut file = File::options()
            .create(true)
            .write(true)
            .open(self.get_file_path(sess.id()))
            .unwrap();
        let sess_ser = serde_json::to_string(&sess).unwrap();
        return file.write_all(sess_ser.as_bytes());
    }

    pub fn get_session(&self, id: session::SessionId) -> session::Session {
        let mut file = File::options()
            .read(true)
            .open(self.get_file_path(id))
            .unwrap();
        let sess_ser = fs::read_to_string(self.get_file_path(id)).unwrap();
        let sess: session::Session = serde_json::from_str(&sess_ser).unwrap();
        return sess;
    }

    pub fn del_session(&self, id: session::SessionId) -> Result<(), std::io::Error> {
        return fs::remove_file(self.get_file_path(id));
    }
}
