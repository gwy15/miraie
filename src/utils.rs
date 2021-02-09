macro_rules! def_req {
    ($(
        $(#[$meta:meta])?
        $vis:vis $name:ident: $type:ty,
    )*) => {
        #[derive(Debug, Serialize)]
        struct Request {
            $(
                $(#[$meta])?
                $vis $name: $type,
            )*
        }
    }
}

macro_rules! def_resp {
    ($(
        $(#[$meta:meta])?
        $vis:vis $name:ident: $type:ty,
    )*) => {
        #[derive(Debug, Deserialize)]
        struct Response {
            code: i32,
            $(
                $(#[$meta])?
                $vis $name: $type,
            )*
        }
        #[allow(unused)]
        impl Response {
            pub fn ok(&self) -> std::result::Result<(), $crate::Error> {
                if self.code != 0 {
                    debug!("resp fail: {:?}", self);
                    return Err($crate::Error::Status(self.code));
                }
                Ok(())
            }
        }
    }
}
