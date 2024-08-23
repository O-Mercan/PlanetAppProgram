use crate::{error::PlanetAppError::InvalidInstruction,
state::{Explorer, Planet} };
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug, PartialEq, Clone)]
pub enum PlanetAppInstruction{
    NewExplorer{data: Explorer},
    NewPlanet{data: Planet},
    Confirme,
    AddIfConfirme,
    
}

impl PlanetAppInstruction{
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("unpack");
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        msg!("split ");
        Ok( match tag {
            0 => Self::NewExplorer{data: Explorer::try_from_slice(&rest)?},
            1 => Self::NewPlanet{data: Planet::try_from_slice(&rest)?}, //deserialize etme işlemi try_from_slice       
            2 => Self::Confirme,
            3 => Self::AddIfConfirme,
            _ => return Err(InvalidInstruction.into()),
        })
    }
}