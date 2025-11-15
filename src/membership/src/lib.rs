/*!
 * Membership Module - Global member management and cross-hub transfers
 */

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::{Currency, generate_id, HubId, MemberId, Region};

/// Member profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: MemberId,
    pub name: String,
    pub region: Region,
    pub home_hub: HubId,
    pub join_date: DateTime<Utc>,
    pub membership_tier: MembershipTier,
    pub portable_id: String, // Global portable identity
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MembershipTier {
    Basic,
    Professional,
    Steward,
}

/// Membership dues structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuesStructure {
    pub region: Region,
    pub amount: u128,
    pub currency: Currency,
    pub period: DuesPeriod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DuesPeriod {
    Monthly,
    Quarterly,
    Annual,
}

/// Cross-hub transfer record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubTransfer {
    pub member_id: MemberId,
    pub from_hub: HubId,
    pub to_hub: HubId,
    pub transfer_date: DateTime<Utc>,
    pub reason: String,
}

pub struct MembershipSystem {
    region: Region,
    members: HashMap<MemberId, Member>,
    transfers: Vec<HubTransfer>,
}

impl MembershipSystem {
    pub fn new(region: &str) -> Result<Self> {
        info!("Initializing Membership System for region: {}", region);

        Ok(Self {
            region: Region::from_str(region),
            members: HashMap::new(),
            transfers: Vec::new(),
        })
    }

    /// Join the global commons with region-specific dues
    pub fn global_join(
        &mut self,
        name: String,
        dues: u128,
        region: Region,
    ) -> Result<MemberId> {
        info!("Processing membership for {} in region: {}", name, region);

        let member_id = generate_id();
        let portable_id = format!("SC-{}", member_id);

        let member = Member {
            id: member_id,
            name: name.clone(),
            region: region.clone(),
            home_hub: generate_id(), // Assign home hub
            join_date: Utc::now(),
            membership_tier: MembershipTier::Basic,
            portable_id: portable_id.clone(),
        };

        self.members.insert(member_id, member);

        info!("Member {} joined with portable ID: {}", name, portable_id);

        Ok(member_id)
    }

    /// Transfer member between hubs seamlessly
    pub fn cross_hub_transfer(
        &mut self,
        member_id: MemberId,
        from_hub: HubId,
        to_hub: HubId,
    ) -> Result<Member> {
        info!("Transferring member {} from hub {} to {}", member_id, from_hub, to_hub);

        let member = self.members
            .get_mut(&member_id)
            .ok_or_else(|| anyhow::anyhow!("Member not found"))?;

        // Record the transfer
        let transfer = HubTransfer {
            member_id,
            from_hub,
            to_hub,
            transfer_date: Utc::now(),
            reason: "Hub relocation".to_string(),
        };

        self.transfers.push(transfer);

        // Update member's home hub
        member.home_hub = to_hub;

        info!("Transfer completed. Member now at hub: {}", to_hub);

        Ok(member.clone())
    }

    /// Get localized dues structure
    pub fn get_dues_structure(&self, region: Region) -> DuesStructure {
        match region {
            Region::LA | Region::NYC | Region::Atlanta => DuesStructure {
                region,
                amount: 5000, // $50.00/month
                currency: Currency::USD,
                period: DuesPeriod::Monthly,
            },
            Region::Mumbai => DuesStructure {
                region,
                amount: 200000, // ₹2000/month
                currency: Currency::INR,
                period: DuesPeriod::Monthly,
            },
            Region::Berlin | Region::London => DuesStructure {
                region,
                amount: 4500, // €45.00/month
                currency: Currency::EUR,
                period: DuesPeriod::Monthly,
            },
            _ => DuesStructure {
                region,
                amount: 5000,
                currency: Currency::USD,
                period: DuesPeriod::Monthly,
            },
        }
    }

    /// Get member by ID
    pub fn get_member(&self, member_id: MemberId) -> Option<&Member> {
        self.members.get(&member_id)
    }

    /// Get all members
    pub fn get_all_members(&self) -> Vec<&Member> {
        self.members.values().collect()
    }

    /// Get transfer history for a member
    pub fn get_transfer_history(&self, member_id: MemberId) -> Vec<&HubTransfer> {
        self.transfers
            .iter()
            .filter(|t| t.member_id == member_id)
            .collect()
    }

    /// Upgrade membership tier
    pub fn upgrade_tier(&mut self, member_id: MemberId, tier: MembershipTier) -> Result<()> {
        let member = self.members
            .get_mut(&member_id)
            .ok_or_else(|| anyhow::anyhow!("Member not found"))?;

        member.membership_tier = tier;

        info!("Member {} upgraded to tier: {:?}", member_id, member.membership_tier);

        Ok(())
    }
}

impl Default for MembershipSystem {
    fn default() -> Self {
        Self::new("LA").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_join() {
        let mut membership = MembershipSystem::new("LA").unwrap();

        let member_id = membership
            .global_join("Test User".to_string(), 5000, Region::LA)
            .unwrap();

        assert!(membership.get_member(member_id).is_some());
    }

    #[test]
    fn test_cross_hub_transfer() {
        let mut membership = MembershipSystem::new("LA").unwrap();

        let member_id = membership
            .global_join("Test User".to_string(), 5000, Region::LA)
            .unwrap();

        let from_hub = uuid::Uuid::new_v4();
        let to_hub = uuid::Uuid::new_v4();

        let updated = membership
            .cross_hub_transfer(member_id, from_hub, to_hub)
            .unwrap();

        assert_eq!(updated.home_hub, to_hub);
    }

    #[test]
    fn test_dues_structure() {
        let membership = MembershipSystem::new("LA").unwrap();

        let dues_us = membership.get_dues_structure(Region::LA);
        assert_eq!(dues_us.currency, Currency::USD);

        let dues_india = membership.get_dues_structure(Region::Mumbai);
        assert_eq!(dues_india.currency, Currency::INR);
    }

    #[test]
    fn test_tier_upgrade() {
        let mut membership = MembershipSystem::new("LA").unwrap();

        let member_id = membership
            .global_join("Test User".to_string(), 5000, Region::LA)
            .unwrap();

        membership.upgrade_tier(member_id, MembershipTier::Professional).unwrap();

        let member = membership.get_member(member_id).unwrap();
        assert_eq!(member.membership_tier, MembershipTier::Professional);
    }
}
