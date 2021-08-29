mod response_types;
mod utils;

use log::info;
use response_types::{PlaylistAddItemResponse, PlaylistInfo, PlaylistTracksResponse};
use std::collections::HashMap;
use utils::raxios::RaxiosClient;

const SPOTIFY_URL: &'static str = "https://api.spotify.com/v1";

pub struct PlaylistCloner {
    playlist_id: &'static str,
    user_id: &'static str,
    raxios: RaxiosClient,

    // stored data
    playlist_name: Option<String>,
    playlist_description: Option<String>,
    playlist_tracks: Vec<String>,
}

impl PlaylistCloner {
    pub fn new(playlist_id: &'static str, user_id: &'static str, token: String) -> Self {
        let mut raxios = RaxiosClient::new();
        raxios.set_token(&token);

        let playlist_cloner = PlaylistCloner {
            playlist_id,
            user_id,
            raxios,

            playlist_name: None,
            playlist_description: None,
            playlist_tracks: Vec::new(),
        };

        return playlist_cloner;
    }

    pub async fn clone_playlist(&mut self) -> Result<(), reqwest::Error> {
        self.get_playlist_details().await;
        self.create_playlist_from_tracks().await?;

        Ok(())
    }

    pub async fn get_playlist_details(&mut self) {
        info!("Fetch playlist information {}", self.playlist_id);
        let basic_info = self.get_playlist_basic_info_request().await.unwrap();
        self.playlist_name = Some(basic_info.name);
        self.playlist_description = basic_info.description;

        info!("Fetch tracks from playlist {}", self.playlist_id);
        let data = self.get_playlist_tracks_request(None).await.unwrap();

        let mut next = data.next;

        let mut songs = data
            .items
            .into_iter()
            .map(|item| item.track.id)
            .collect::<Vec<String>>();

        while next.is_some() {
            let data = self
                .get_playlist_tracks_request(next.as_ref())
                .await
                .unwrap();

            next = data.next;

            let mut items = data
                .items
                .into_iter()
                .map(|item| item.track.id)
                .collect::<Vec<String>>();
            songs.append(&mut items);
        }

        self.playlist_tracks = songs;
    }

    pub async fn create_playlist_from_tracks(&self) -> Result<(), reqwest::Error> {
        info!(
            "creating playlist - name: {}, tracks_length: {}",
            self.playlist_name.as_ref().unwrap(),
            self.playlist_tracks.len()
        );

        let create_url = format!("{}/users/{}/playlists", SPOTIFY_URL, self.user_id);
        let mut create_req_body = HashMap::new();

        let name = &format!("{} - cloned", self.playlist_name.as_ref().unwrap());
        create_req_body.insert("name".to_owned(), name);
        if self.playlist_description.is_some() {
            create_req_body.insert(
                "description".to_owned(),
                self.playlist_description.as_ref().unwrap(),
            );
        }

        info!(
            "sending create request - {} {:?}",
            create_url, &create_req_body
        );

        let data = self
            .raxios
            .post::<PlaylistInfo, HashMap<String, &String>>(
                &create_url,
                Some(create_req_body),
                None,
            )
            .await
            .expect(&format!("Failed to create playlist"));

        info!("playlist basic info added");
        let tracks = self
            .playlist_tracks
            .clone()
            .into_iter()
            .map(|item| format!("spotify:track:{}", item))
            .collect::<Vec<String>>();

        let add_url = format!("{}/playlists/{}/tracks", SPOTIFY_URL, data.id);
        let mut add_req_body = HashMap::new();
        add_req_body.insert("uris".to_owned(), tracks);

        info!(
            "sending add items request - {} {:?}",
            add_url, &add_req_body
        );

        self.raxios
            .post::<PlaylistAddItemResponse, HashMap<String, Vec<String>>>(
                &add_url,
                Some(add_req_body),
                None,
            )
            .await?;

        info!("playlist tracks added");
        Ok(())
    }

    async fn get_playlist_tracks_request(
        &self,
        link: Option<&String>,
    ) -> Result<PlaylistTracksResponse, reqwest::Error> {
        let url = match link {
            Some(link) => link.to_owned(),
            None => format!("{}/playlists/{}/tracks", SPOTIFY_URL, self.playlist_id),
        };

        let data = self
            .raxios
            .get::<PlaylistTracksResponse>(&url, None)
            .await?;
        Ok(data)
    }

    async fn get_playlist_basic_info_request(&self) -> Result<PlaylistInfo, reqwest::Error> {
        let url = format!("{}/playlists/{}", SPOTIFY_URL, self.playlist_id);

        let data = self.raxios.get::<PlaylistInfo>(&url, None).await?;
        Ok(data)
    }
}
