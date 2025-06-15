use crate::packets::packet::PacketError;
use crate::utils::utils::extend_with_string;
use vintor::{encode, EncodeError};


pub fn create_known_packs_response() -> Result<Vec<u8>,PacketError>
{
    let mut response = Vec::new();
    let namespace = String::from("minecraft");
    let id = String::from("core");
    let version = String::from("1.21.5");

    let mut inner = Vec::new();
    let array_lengt = encode(1)?;
    let packet_id = encode(14)?;
    inner.extend(packet_id); 
    inner.extend(array_lengt);
    
    extend_with_string(&mut inner, namespace)?;
    extend_with_string(&mut inner, id)?;
    extend_with_string(&mut inner, version)?;
    let inner_length = encode(inner.len() as i32)?;
    response.extend(inner_length);
    response.extend(inner);

    Ok(response)
}
pub fn extend_with_entity(bytes: &mut Vec<u8>, entity: String) -> Result<(),PacketError>
{
    let zero = encode(0)?;
    extend_with_string(bytes, entity)?;
    bytes.extend(zero);
    Ok(())
}
pub fn create_biomes_registry_data_packet() -> Result<Vec<u8>, PacketError>
{
    let mut inner = Vec::new();
    let mut response= Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:worldgen/biome");
    let entities : [String;4] = [String::from("minecraft:badlands"),
                                String::from("minecraft:bamboo_jungle"),
                                String::from("minecraft:basalt_deltas"),
                                String::from("minecraft:beach")];

    extend_with_string(&mut inner, registry)?;

    let array_len = encode(4)?;
    inner.extend(array_len);
    
    for ent in entities
    {
        extend_with_entity(&mut inner, ent)?;
    }
    
    let inner_len = encode(inner.len()as i32) ?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)

}
pub fn create_dimensions_type_registry_data_packet() -> Result<Vec<u8>, PacketError>
{
    let mut inner = Vec::new();
    let mut response= Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:dimension_type");
    let entities : [String;4] = [String::from("minecraft:overworld"),
                                String::from("minecraft:overworld_caves"),
                                String::from("minecraft:the_end"),
                                String::from("minecraft:the_nether")];

    extend_with_string(&mut inner, registry)?;

    let array_len = encode(4)?;
    inner.extend(array_len);
    
    for ent in entities
    {
        extend_with_entity(&mut inner, ent)?;
    }
    
    let inner_len = encode(inner.len()as i32) ?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)

}

pub fn create_painting_variant_registry_data_packet() -> Result<Vec<u8>, PacketError>
{
    let mut inner = Vec::new();
    let mut response= Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:painting_variant");
    let entities : [String;4] = [String::from("minecraft:aztec"),
                                String::from("minecraft:aztec2"),
                                String::from("minecraft:alban"),
                                String::from("minecraft:backyard")];

    extend_with_string(&mut inner, registry)?;

    let array_len = encode(4)?;
    inner.extend(array_len);
    
    for ent in entities
    {
        extend_with_entity(&mut inner, ent)?;
    }
    
    let inner_len = encode(inner.len()as i32) ?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)

}

pub fn create_wolf_variant_registry_data_packet() -> Result<Vec<u8>, PacketError>
{
    let mut inner = Vec::new();
    let mut response= Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:wolf_variant");
    let entities : [String;4] = [String::from("minecraft:ashen"),
                                String::from("minecraft:black"),
                                String::from("minecraft:chestnut"),
                                String::from("minecraft:pale")];

    extend_with_string(&mut inner, registry)?;

    let array_len = encode(4)?;
    inner.extend(array_len);
    
    for ent in entities
    {
        extend_with_entity(&mut inner, ent)?;
    }
    
    let inner_len = encode(inner.len()as i32) ?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)

}

pub fn create_damage_type_variant_registry_data_packet() -> Result<Vec<u8>, PacketError>
{
    let mut inner = Vec::new();
    let mut response= Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:damage_type");
    let entities : [String;4] = [String::from("minecraft:fall"),
                                String::from("minecraft:arrow"),
                                String::from("minecraft:generic"),
                                String::from("minecraft:mob_attack")];

    extend_with_string(&mut inner, registry)?;

    let array_len = encode(4)?;
    inner.extend(array_len);
    
    for ent in entities
    {
        extend_with_entity(&mut inner, ent)?;
    }
    
    let inner_len = encode(inner.len()as i32) ?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)

}

pub fn create_finish_configuration_packet() -> Result<Vec<u8>,PacketError>
{
    let mut response = Vec::new();
    let packet_length = encode(1)?;
    let packet_id = encode(3)?;
    
    response.extend(packet_length);
    response.extend(packet_id);

    Ok(response)
}
pub fn create_cat_variant_registry_data_packet() -> Result<Vec<u8>, PacketError> {
    let mut inner = Vec::new();
    let mut response = Vec::new();
    let packet_id = encode(7)?; // Registry data packet id
    inner.extend(packet_id);
    let registry = String::from("minecraft:cat_variant");
    let entities: [String; 3] = [
        String::from("minecraft:black"),
        String::from("minecraft:calico"),
        String::from("minecraft:white"),
    ];

    extend_with_string(&mut inner, registry)?;
    let array_len = encode(entities.len() as i32)?;
    inner.extend(array_len);
    for ent in entities {
        extend_with_entity(&mut inner, ent)?;
    }

    let inner_len = encode(inner.len() as i32)?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)
}

pub fn create_chicken_variant_registry_data_packet() -> Result<Vec<u8>, PacketError> {
    let mut inner = Vec::new();
    let mut response = Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:chicken_variant");
    let entities: [String; 3] = [
        String::from("minecraft:temperate"),
        String::from("minecraft:cold"),
        String::from("minecraft:warm"),
    ];

    extend_with_string(&mut inner, registry)?;
    let array_len = encode(entities.len() as i32)?;
    inner.extend(array_len);
    for ent in entities {
        extend_with_entity(&mut inner, ent)?;
    }

    let inner_len = encode(inner.len() as i32)?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)
}

pub fn create_cow_variant_registry_data_packet() -> Result<Vec<u8>, PacketError> {
    let mut inner = Vec::new();
    let mut response = Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:cow_variant");
    let entities: [String; 3] = [
        String::from("minecraft:temperate"),
        String::from("minecraft:cold"),
        String::from("minecraft:warm"),
    ];

    extend_with_string(&mut inner, registry)?;
    let array_len = encode(entities.len() as i32)?;
    inner.extend(array_len);
    for ent in entities {
        extend_with_entity(&mut inner, ent)?;
    }

    let inner_len = encode(inner.len() as i32)?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)
}

pub fn create_frog_variant_registry_data_packet() -> Result<Vec<u8>, PacketError> {
    let mut inner = Vec::new();
    let mut response = Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:frog_variant");
    let entities: [String; 3] = [
        String::from("minecraft:temperate"),
        String::from("minecraft:cold"),
        String::from("minecraft:warm"),
    ];

    extend_with_string(&mut inner, registry)?;
    let array_len = encode(entities.len() as i32)?;
    inner.extend(array_len);
    for ent in entities {
        extend_with_entity(&mut inner, ent)?;
    }

    let inner_len = encode(inner.len() as i32)?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)
}

pub fn create_pig_variant_registry_data_packet() -> Result<Vec<u8>, PacketError> {
    let mut inner = Vec::new();
    let mut response = Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:pig_variant");
    let entities: [String; 3] = [String::from("minecraft:temperate"),
                                String::from("minecraft:warm"),
                                String::from("minecraft:cold")];

    extend_with_string(&mut inner, registry)?;
    let array_len = encode(entities.len() as i32)?;
    inner.extend(array_len);
    for ent in entities {
        extend_with_entity(&mut inner, ent)?;
    }

    let inner_len = encode(inner.len() as i32)?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)
}

pub fn create_wolf_sound_variant_registry_data_packet() -> Result<Vec<u8>, PacketError> {
    let mut inner = Vec::new();
    let mut response = Vec::new();
    let packet_id = encode(7)?;
    inner.extend(packet_id);
    let registry = String::from("minecraft:wolf_sound_variant");
    let entities: [String; 4] = [
        String::from("minecraft:big"),
        String::from("minecraft:classic"),
        String::from("minecraft:cute"),
        String::from("minecraft:puglin"),
    ];

    extend_with_string(&mut inner, registry)?;
    let array_len = encode(entities.len() as i32)?;
    inner.extend(array_len);
    for ent in entities {
        extend_with_entity(&mut inner, ent)?;
    }

    let inner_len = encode(inner.len() as i32)?;
    response.extend(inner_len);
    response.extend(inner);
    Ok(response)
}

