use scrypto::prelude::*;

pub fn assert_within_bounds<T: PartialOrd + Display>(
    value: T,
    min_value: T,
    max_value: T,
    value_name: &str,
) {
    assert!(
        min_value <= value && value <= max_value,
        "The provided {} is not in the allowed interval.\n Allowed: [{}, {}]\n Provided: {}",
        value_name,
        min_value.to_string(),
        max_value.to_string(),
        value.to_string()
    );
}

pub fn assert_fee_rate_within_bounds(fee_rate: Decimal, fee_rate_max: Decimal, fee_name: &str) {
    assert_within_bounds(fee_rate, dec!(0), fee_rate_max, fee_name);
}

pub fn sort_buckets(a_bucket: Bucket, b_bucket: Bucket) -> (Bucket, Bucket) {
    if a_bucket.resource_address() < b_bucket.resource_address() {
        return (a_bucket, b_bucket);
    }
    (b_bucket, a_bucket)
}

pub fn assert_components_are_approved(
    metadata_key: &str,
    component_addresses: Vec<ComponentAddress>,
) {
    let metadata_package_addresses = package_addresses_from_metadata(&metadata_key);

    for component_address in component_addresses {
        let blueprint_id = ScryptoVmV1Api::object_get_blueprint_id(&component_address.into());
        assert!(
            metadata_package_addresses.contains(&blueprint_id.package_address),
            "The package address of {} is not approved under the key {} in the pool package metadata.",
            Runtime::bech32_encode_address(component_address),
            metadata_key
        );
    }
}

pub fn package_addresses_from_metadata(key: &str) -> Vec<PackageAddress> {
    let own_package: Package = Runtime::package_address().into();
    let metadata_value: Option<Vec<GlobalAddress>> = own_package.get_metadata(key).ok().flatten();
    metadata_value
        .into_iter()
        .flatten()
        .filter_map(|address| address.try_into().ok())
        .collect()
}
