use crate::{
    constants::*, 
    error::*, 
    extentions::Discriminator
};

pub trait EventParser: Discriminator + Sized {
    fn parse_instruction(data: &[u8]) -> SdkResult<Self> 
    where
        Self: borsh::BorshDeserialize
    {
        let header_size: usize = ANCHOR_DISCRIMINATOR_SIZE * 2;

        if data.len() < header_size {
            return Err(SdkError::ParseEvent(ParseEventError::InvalidData));
        }

        if data[..ANCHOR_DISCRIMINATOR_SIZE] != EVENT_DISCRIMINATOR {
            return Err(SdkError::ParseEvent(ParseEventError::InvalidInstructionDiscriminator));
        }

        if data[ANCHOR_DISCRIMINATOR_SIZE..header_size] != Self::DISCRIMINATOR {
            return Err(SdkError::ParseEvent(ParseEventError::InvalidDiscriminator));
        }

        Self::try_from_slice(&data[header_size..])
            .map_err(Into::into)
    }
}