use sqlx::types::ipnetwork::IpNetwork;
use std::error::Error;

pub fn parse_client_ip(remote_addr: &Option<&str>) -> Result<IpNetwork, Box<dyn Error>> {
    if let Some(ip_address) = remote_addr {
        let parsed = ip_address.parse()?;
        return Ok(parsed);
    }

    Err("Unable to parse client ip".into())
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
