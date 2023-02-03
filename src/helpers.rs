use crate::errors::InternalError;
use sqlx::types::ipnetwork::IpNetwork;

pub fn parse_client_ip(remote_addr: &Option<&str>) -> Result<IpNetwork, InternalError> {
    let ip_address = remote_addr.ok_or(InternalError::ParseIpError(
        "Failed to retrieve remote address.",
    ))?;

    let parsed = ip_address
        .parse()
        .map_err(|_| InternalError::ParseIpError("Failed to parse valid remote address."))?;

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::{internet::en::IPv4, lorem::en::Word};
    use fake::Fake;

    #[test]
    fn parsed_ip_address_correctly() {
        let fake_ip: String = IPv4().fake();
        let remote_addr = Some(fake_ip.as_str());
        let parsed = parse_client_ip(&remote_addr);

        assert!(parsed.is_ok());
    }

    #[test]
    fn fails_to_parse_malformed_ip_address() {
        let malformed_ip = Word().fake();
        let remote_addr = Some(malformed_ip);
        let parsed = parse_client_ip(&remote_addr);

        assert!(parsed.is_err());
    }

    #[test]
    fn fails_to_parse_ip_address_if_remote_addr_is_none() {
        let remote_addr = None;
        let parsed = parse_client_ip(&remote_addr);

        assert!(parsed.is_err());
    }
}
