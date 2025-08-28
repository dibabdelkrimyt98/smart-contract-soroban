use soroban_sdk::{contracterror};

#[contracterror]
#[derive(Debug, Copy, Clone)]
pub enum ComplianceError {
    InvalidCertificate = 1,
    Unauthorized = 2,
    CertificateExpired = 3,
}
