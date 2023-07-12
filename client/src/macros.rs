#![macro_use]

#[macro_export]
/// Dentro de un loop, verifica un Result, si está Ok, devuelve el valor, si es un Err, continua
macro_rules! pass_err {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => continue
        }
    };
}
