#![cfg(feature = "parse-events")]

use borsh::BorshSerialize;
use lastbuy_sdk::{ANCHOR_DISCRIMINATOR_SIZE, EVENT_DISCRIMINATOR, GRADUATION_EVENT_DISCRIMINATOR, events::{EventParser, GraduationEvent}};
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