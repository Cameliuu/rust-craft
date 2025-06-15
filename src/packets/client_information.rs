use crate::packets::packet::PacketError;
use vintor::{decode,DecodeError};
#[derive(Debug)]
pub struct ClientInformationPackage {
    locale: String,
    view_distance: u8,
    chat_mode: i32,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: i32,
    enable_text_filtering: bool,
    allow_server_listings: bool,
    particle_status: i32
}

impl ClientInformationPackage{

    pub fn from_bytes(bytes: &[u8]) -> Result<ClientInformationPackage,PacketError>
    {
        let mut cursor =0;
        let (locale_len,locale_read) = decode(bytes)?;
        cursor += locale_read;

        let locale_bytes=&bytes[cursor..locale_len as usize];
        let locale = std::str::from_utf8(locale_bytes).expect("[ ! ] FAILED TO PARSE LOCALE STRING").to_string();
        cursor += locale_len as usize;

        let view_distance = bytes[cursor];
        cursor += 1;

        let (chat_mode, chat_mode_read) = decode(&bytes[cursor..])?; 
        cursor += chat_mode_read as usize;

        let chat_colors_bytes = &bytes[cursor..];
        let chat_colors = chat_colors_bytes[0] != 0;
        cursor += 1;

        let displayed_skin_parts = bytes[cursor];
        cursor += 1;

        let (main_hand,main_hand_read) = decode(&bytes[cursor..])?;
       cursor += main_hand_read as usize; 

        let enable_text_filtering = bytes[cursor] != 0;
        cursor +=1;

        let allow_server_listings = bytes[cursor] !=0;
        cursor +=1;

        let (particle_status, particle_status_read )= decode(&bytes[cursor..])?;

        Ok(ClientInformationPackage
            {
                locale:locale,
                view_distance:view_distance,
                chat_mode:chat_mode,
                chat_colors: chat_colors,
                displayed_skin_parts: displayed_skin_parts,
                main_hand: main_hand,
                enable_text_filtering: enable_text_filtering,
                allow_server_listings: allow_server_listings,
                particle_status: particle_status

            })
    }
}
