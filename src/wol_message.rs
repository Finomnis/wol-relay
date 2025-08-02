use wol::MacAddr6;

pub const WOL_HEADER_SIZE: usize = 6;
pub const WOL_MAC_SIZE: usize = 6;
pub const WOL_MAC_REPETITIONS: usize = 16;

pub const WOL_MAX_SIZE: usize = WOL_HEADER_SIZE + WOL_MAC_SIZE * WOL_MAC_REPETITIONS;

fn advance_buffer_by<'a, const N: usize>(msg: &mut &'a [u8]) -> Option<&'a [u8; N]> {
    let (data, rest) = msg.split_first_chunk::<N>()?;
    *msg = rest;
    Some(data)
}

pub fn parse_wol_message(mut msg: &[u8]) -> Option<MacAddr6> {
    let header = advance_buffer_by::<WOL_HEADER_SIZE>(&mut msg)?;
    if header != &[0xff; WOL_HEADER_SIZE] {
        return None;
    }

    let mac_addr: MacAddr6 = advance_buffer_by::<WOL_MAC_SIZE>(&mut msg)?
        .to_owned()
        .into();

    for _ in 1..WOL_MAC_REPETITIONS {
        let mac_addr_repeat: MacAddr6 = advance_buffer_by::<WOL_MAC_SIZE>(&mut msg)?
            .to_owned()
            .into();

        if mac_addr_repeat != mac_addr {
            return None;
        }
    }

    Some(mac_addr)
}
