#![cfg(feature = "parse-events")]

use borsh::BorshSerialize;
use lastbuy_sdk::{ANCHOR_DISCRIMINATOR_SIZE, EVENT_DISCRIMINATOR, GRADUATION_EVENT_DISCRIMINATOR, events::{EventParser, GraduationEvent, LaunchEvent}};
use solana_pubkey::Pubkey;

#[test]
#[ignore]
fn test_event_parser() -> Result<(), Box<dyn std::error::Error>> {
    let graduation_event = GraduationEvent {
        curve: Pubkey::new_unique(),
        mint: Pubkey::new_unique(),
        timestamp: 123123123
    };

    let mut raw_data = Vec::<u8>::new();

    raw_data.extend(GRADUATION_EVENT_DISCRIMINATOR);
    graduation_event.serialize(&mut raw_data)?;

    let mut instruction_data = Vec::with_capacity(ANCHOR_DISCRIMINATOR_SIZE + raw_data.len());
    // emulate emitted ix
    instruction_data.extend(EVENT_DISCRIMINATOR);
    instruction_data.extend(raw_data);

    let graduation_event_from_ix_data = GraduationEvent::parse_instruction(&instruction_data)?;

    assert_eq!(graduation_event, graduation_event_from_ix_data);

    Ok(())
}

#[test]
#[ignore]
fn test_parse_launch_event() -> Result<(), Box<dyn std::error::Error>> {
    let raw_event_ix_data: Vec<u8> = vec![
        228, 69, 165, 46, 81, 203, 154, 29, 27, 193, 47, 130, 115, 92, 239, 94,
        191, 209, 98, 144, 139, 179, 44, 136, 248, 242, 117, 84, 29, 134, 18, 229,
        248, 205, 107, 66, 75, 114, 169, 173, 188, 191, 173, 215, 50, 77, 46, 22,
        123, 50, 236, 183, 181, 202, 103, 87, 72, 30, 215, 149, 185, 142, 144, 188,
        30, 46, 171, 100, 174, 136, 177, 79, 12, 158, 112, 185, 66, 38, 174, 219,
        19, 188, 182, 57, 54, 213, 232, 217, 86, 242, 223, 218, 79, 219, 233, 241,
        153, 241, 144, 11, 170, 180, 31, 30, 1, 10, 171, 99, 236, 90, 208, 229, 24,
        242, 253, 106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 202, 154, 59, 0, 0,
        0, 0, 10, 0, 0, 0, 84, 101, 115, 116, 32, 84, 111, 107, 101, 110, 4, 0, 0,
        0, 84, 69, 83, 84, 29, 0, 0, 0, 104, 116, 116, 112, 115, 58, 47, 47, 101,
        120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 116, 101, 115, 116, 46,
        106, 115, 111, 110
    ];

    let launch_event = LaunchEvent::parse_instruction(&raw_event_ix_data)?;
    
    println!("{:#?}", launch_event);

    Ok(())
}