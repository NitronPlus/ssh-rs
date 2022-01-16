use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use rand::Rng;
use rand::rngs::OsRng;
use crate::error::{SshErrorKind, SshResult};
use crate::{Client, Config, SshError};
use crate::encryption::ChaCha20Poly1305;
use crate::global::{CLIENT, CONFIG, ENCRYPTION_KEY};


pub(crate) fn write(client: Arc<Mutex<Client>>, v: &[u8]) -> SshResult<()> {
    match client.lock() {
        Ok(mut s) => Ok(s.write(v)?),
        Err(e) => {
            log::error!("Get client mutex error, error info: {:?}", e);
            Err(SshError::from(SshErrorKind::MutexError))
        }
    }
}

pub(crate) fn read(client: Arc<Mutex<Client>>) -> SshResult<Vec<Vec<u8>>> {
    match client.lock() {
        Ok(mut v) => Ok(v.read()?),
        Err(e) => {
            log::error!("Get client mutex error, error info: {:?}", e);
            Err(SshError::from(SshErrorKind::MutexError))
        }
    }
}

pub(crate) fn from_utf8(v: Vec<u8>) -> SshResult<String> {
    match String::from_utf8(v) {
        Ok(v) => Ok(v),
        Err(e) => {
            log::error!("Byte to utf8 string error, error info: {:?}", e);
            Err(SshError::from(SshErrorKind::FromUtf8Error))
        }
    }
}


pub fn unlock<T>(guard: MutexGuard<'static, T>) {
    drop(guard);
}


pub(crate) fn update_client(v: Option<Mutex<Client>>) {
    unsafe {
        CLIENT = v
    }
}

pub(crate) fn client() -> SshResult<MutexGuard<'static, Client>> {
    unsafe {
        match &mut CLIENT {
            None => {
                log::error!("Client null pointer");
                Err(SshError::from(SshErrorKind::ClientNullError))
            }
            Some(v) => {
                match v.lock() {
                    Ok(c) => Ok(c),
                    Err(e) => {
                        log::error!("Get client mutex error, error info: {:?}", e);
                        Err(SshError::from(SshErrorKind::MutexError))
                    }
                }
            }
        }
    }
}


pub(crate) fn update_config(v: Option<Mutex<Config>>) {
    unsafe {
        CONFIG = v;
    }
}

pub(crate) fn config() -> SshResult<MutexGuard<'static, Config>> {
    unsafe {
         match &mut CONFIG {
            None => {
                log::error!("Config null pointer");
                Err(SshError::from(SshErrorKind::ConfigNullError))
            }
            Some(v) => {
                match v.lock() {
                    Ok(c) => Ok(c),
                    Err(e) => {
                        log::error!("Get config mutex error, error info: {:?}", e);
                        Err(SshError::from(SshErrorKind::MutexError))
                    }
                }
            }
        }
    }
}


pub(crate) fn encryption_key() -> Result<&'static mut ChaCha20Poly1305, SshError>  {
    unsafe {
        match &mut ENCRYPTION_KEY {
            None => {
                log::error!("Encrypted null pointer");
                Err(SshError::from(SshErrorKind::EncryptionNullError))
            },
            Some(v) => Ok(v)
        }
    }
}
pub(crate) fn update_encryption_key(v: Option<ChaCha20Poly1305>) {
    unsafe {
        ENCRYPTION_KEY = v
    }
}


// 十六位随机数
pub(crate) fn cookie() -> Vec<u8> {
    let cookie: [u8; 16] = OsRng.gen();
    cookie.to_vec()
}

pub(crate) fn vec_u8_to_string(v: Vec<u8>, pat: &str) -> SshResult<Vec<String>> {
    let result = from_utf8(v)?;
    let r: Vec<&str> = result.split(pat).collect();
    let mut vec = vec![];
    for x in r {
        vec.push(x.to_string())
    }
    Ok(vec)
}