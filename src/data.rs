pub mod entity {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Artist {
        name: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Album {
        name: String,
        artists: Vec<Artist>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Track {
        name: String,
        href: String,
        popularity: u32,
        album: Album,
    }
}

pub mod dto {
    pub mod responses {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct ListResponse<T> {
            pub items: Vec<T>
        }
    }
}