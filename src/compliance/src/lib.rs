/*!
 * Compliance Module - Global jurisdiction adaptation and legal compliance
 */

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use utils::Region;

/// Compliance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub region: Region,
    pub action: String,
    pub compliant: bool,
    pub requirements: Vec<String>,
    pub warnings: Vec<String>,
}

/// Legal action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    DataCollection,
    DataStorage,
    MemberRegistration,
    Payment,
    ContentDistribution,
    EmploymentContract,
    Other(String),
}

/// Wage scale configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WageScale {
    pub region: Region,
    pub minimum_hourly: u128,
    pub union_compatible: bool,
    pub living_wage_certified: bool,
}

/// Union integration rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionRules {
    pub region: Region,
    pub unions: Vec<String>,
    pub mandatory: bool,
    pub collective_bargaining: bool,
}

pub struct ComplianceSystem {
    region: Region,
}

impl ComplianceSystem {
    pub fn new(region: &str) -> Result<Self> {
        info!("Initializing Compliance System for region: {}", region);

        Ok(Self {
            region: Region::from_str(region),
        })
    }

    /// Check jurisdiction-specific compliance
    pub fn check_jurisdiction(&self, region: Region, action: Action) -> Result<ComplianceResult> {
        info!("Checking compliance for {:?} in region: {}", action, region);

        let mut requirements = Vec::new();
        let mut warnings = Vec::new();
        let mut compliant = true;

        match region {
            Region::Berlin | Region::London => {
                // GDPR compliance
                requirements.push("GDPR data protection requirements".to_string());
                requirements.push("Right to be forgotten implementation".to_string());
                requirements.push("Data sovereignty (EU servers)".to_string());

                if matches!(action, Action::DataCollection | Action::DataStorage) {
                    requirements.push("Explicit consent required".to_string());
                    requirements.push("Privacy impact assessment".to_string());
                }
            }
            Region::Mumbai => {
                // Indian IT Act compliance
                requirements.push("IT Act 2000 compliance".to_string());
                requirements.push("Data localization for sensitive data".to_string());

                if matches!(action, Action::Payment) {
                    requirements.push("RBI payment guidelines".to_string());
                }
            }
            Region::LA | Region::NYC | Region::Atlanta => {
                // US compliance
                requirements.push("IRS tax reporting".to_string());

                if matches!(action, Action::DataCollection) {
                    requirements.push("CCPA compliance (California)".to_string());
                }

                if matches!(action, Action::EmploymentContract) {
                    requirements.push("Fair Labor Standards Act".to_string());
                }
            }
            _ => {
                warnings.push("Using default compliance rules".to_string());
            }
        }

        let result = ComplianceResult {
            region,
            action: format!("{:?}", action),
            compliant,
            requirements,
            warnings,
        };

        info!("Compliance check complete: {} requirements", result.requirements.len());

        Ok(result)
    }

    /// Get union integration requirements
    pub fn union_integrate(&self, region: Region) -> Result<UnionRules> {
        info!("Getting union requirements for region: {}", region);

        let rules = match region {
            Region::LA | Region::NYC => UnionRules {
                region,
                unions: vec![
                    "SAG-AFTRA".to_string(),
                    "DGA".to_string(),
                    "WGA".to_string(),
                    "IATSE".to_string(),
                ],
                mandatory: true,
                collective_bargaining: true,
            },
            Region::Berlin | Region::London => UnionRules {
                region,
                unions: vec![
                    "Ver.di (Germany)".to_string(),
                    "BECTU (UK)".to_string(),
                ],
                mandatory: false,
                collective_bargaining: true,
            },
            Region::Mumbai => UnionRules {
                region,
                unions: vec![
                    "FWICE".to_string(),
                    "Cine & TV Artistes' Association".to_string(),
                ],
                mandatory: false,
                collective_bargaining: true,
            },
            _ => UnionRules {
                region,
                unions: vec![],
                mandatory: false,
                collective_bargaining: false,
            },
        };

        Ok(rules)
    }

    /// Get wage scale for region
    pub fn get_wage_scale(&self, region: Region) -> WageScale {
        match region {
            Region::LA | Region::NYC => WageScale {
                region,
                minimum_hourly: 2000, // $20.00/hour
                union_compatible: true,
                living_wage_certified: true,
            },
            Region::Berlin => WageScale {
                region,
                minimum_hourly: 1500, // €15.00/hour
                union_compatible: true,
                living_wage_certified: true,
            },
            Region::Mumbai => WageScale {
                region,
                minimum_hourly: 30000, // ₹300/hour
                union_compatible: true,
                living_wage_certified: true,
            },
            _ => WageScale {
                region,
                minimum_hourly: 1500,
                union_compatible: false,
                living_wage_certified: false,
            },
        }
    }

    /// Verify GDPR compliance (for EU regions)
    pub fn verify_gdpr(&self) -> bool {
        matches!(self.region, Region::Berlin | Region::London)
    }

    /// Get tax reporting requirements
    pub fn get_tax_requirements(&self, region: Region) -> Vec<String> {
        match region {
            Region::LA | Region::NYC | Region::Atlanta => vec![
                "IRS Form 1099 for contractors".to_string(),
                "W-2 for employees".to_string(),
                "Quarterly estimated taxes".to_string(),
            ],
            Region::Berlin => vec![
                "German tax ID (Steuernummer)".to_string(),
                "VAT registration".to_string(),
                "Annual tax returns".to_string(),
            ],
            Region::Mumbai => vec![
                "PAN card requirement".to_string(),
                "GST registration".to_string(),
                "TDS compliance".to_string(),
            ],
            _ => vec!["Consult local tax authority".to_string()],
        }
    }
}

impl Default for ComplianceSystem {
    fn default() -> Self {
        Self::new("LA").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdpr_compliance() {
        let compliance = ComplianceSystem::new("Berlin").unwrap();

        let result = compliance
            .check_jurisdiction(Region::Berlin, Action::DataCollection)
            .unwrap();

        assert!(result.requirements.iter().any(|r| r.contains("GDPR")));
    }

    #[test]
    fn test_us_compliance() {
        let compliance = ComplianceSystem::new("LA").unwrap();

        let result = compliance
            .check_jurisdiction(Region::LA, Action::DataCollection)
            .unwrap();

        assert!(result.requirements.iter().any(|r| r.contains("CCPA") || r.contains("IRS")));
    }

    #[test]
    fn test_union_integration() {
        let compliance = ComplianceSystem::new("LA").unwrap();

        let rules = compliance.union_integrate(Region::LA).unwrap();

        assert!(rules.mandatory);
        assert!(!rules.unions.is_empty());
    }

    #[test]
    fn test_wage_scales() {
        let compliance = ComplianceSystem::new("LA").unwrap();

        let scale_us = compliance.get_wage_scale(Region::LA);
        assert_eq!(scale_us.minimum_hourly, 2000);

        let scale_india = compliance.get_wage_scale(Region::Mumbai);
        assert_eq!(scale_india.minimum_hourly, 30000);
    }

    #[test]
    fn test_gdpr_verification() {
        let compliance_eu = ComplianceSystem::new("Berlin").unwrap();
        assert!(compliance_eu.verify_gdpr());

        let compliance_us = ComplianceSystem::new("LA").unwrap();
        assert!(!compliance_us.verify_gdpr());
    }
}
