/*!
 * Common utilities for Studio Commons
 */

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Supported global regions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Region {
    LA,          // Los Angeles (Pilot)
    NYC,         // New York City
    Atlanta,     // Atlanta
    Mumbai,      // Mumbai, India
    Berlin,      // Berlin, Germany
    Lagos,       // Lagos, Nigeria
    London,      // London, UK
    Tokyo,       // Tokyo, Japan
    Sydney,      // Sydney, Australia
    Custom(String),
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Region::LA => write!(f, "Los Angeles"),
            Region::NYC => write!(f, "New York City"),
            Region::Atlanta => write!(f, "Atlanta"),
            Region::Mumbai => write!(f, "Mumbai"),
            Region::Berlin => write!(f, "Berlin"),
            Region::Lagos => write!(f, "Lagos"),
            Region::London => write!(f, "London"),
            Region::Tokyo => write!(f, "Tokyo"),
            Region::Sydney => write!(f, "Sydney"),
            Region::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl Region {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "LA" => Region::LA,
            "NYC" => Region::NYC,
            "ATLANTA" => Region::Atlanta,
            "MUMBAI" => Region::Mumbai,
            "BERLIN" => Region::Berlin,
            "LAGOS" => Region::Lagos,
            "LONDON" => Region::London,
            "TOKYO" => Region::Tokyo,
            "SYDNEY" => Region::Sydney,
            _ => Region::Custom(s.to_string()),
        }
    }
}

/// Supported currencies for global operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    INR,
    JPY,
    XRP,
    USDC,  // Stablecoin
    EURS,  // Euro stablecoin
}

/// Unique identifiers for various entities
pub type MemberId = Uuid;
pub type ProjectId = Uuid;
pub type HubId = Uuid;
pub type LicenseId = Uuid;
pub type TransactionHash = String;

/// Revenue stream types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamType {
    Rental,
    EquipmentFees,
    Streaming,
    BoxOffice,
    Licensing,
    Grants,
    Other(String),
}

impl fmt::Display for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamType::Rental => write!(f, "Rental"),
            StreamType::EquipmentFees => write!(f, "Equipment Fees"),
            StreamType::Streaming => write!(f, "Streaming"),
            StreamType::BoxOffice => write!(f, "Box Office"),
            StreamType::Licensing => write!(f, "Licensing"),
            StreamType::Grants => write!(f, "Grants"),
            StreamType::Other(s) => write!(f, "{}", s),
        }
    }
}

/// Common result type for all modules
pub type CommonsResult<T> = Result<T>;

/// Generate a new unique ID
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

/// Format currency amount with proper decimal places
pub fn format_currency(amount: u128, currency: &Currency) -> String {
    let divisor = match currency {
        Currency::JPY => 1, // No decimal places for Yen
        _ => 100,           // 2 decimal places for most currencies
    };

    let whole = amount / divisor;
    let fraction = amount % divisor;

    match currency {
        Currency::USD => format!("${}.{:02}", whole, fraction),
        Currency::EUR => format!("€{}.{:02}", whole, fraction),
        Currency::GBP => format!("£{}.{:02}", whole, fraction),
        Currency::INR => format!("₹{}.{:02}", whole, fraction),
        Currency::JPY => format!("¥{}", whole),
        Currency::XRP => format!("{}.{:06} XRP", whole, fraction), // 6 decimals for XRP
        Currency::USDC => format!("${}.{:02} USDC", whole, fraction),
        Currency::EURS => format!("€{}.{:02} EURS", whole, fraction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_from_str() {
        assert_eq!(Region::from_str("LA"), Region::LA);
        assert_eq!(Region::from_str("mumbai"), Region::Mumbai);
    }

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(100050, &Currency::USD), "$1000.50");
        assert_eq!(format_currency(100050, &Currency::EUR), "€1000.50");
    }

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
    }
}
