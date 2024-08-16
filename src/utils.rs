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

pub fn assert_component_is_approved(namespace: &str, component_address: ComponentAddress) {
    let blueprint_id = ScryptoVmV1Api::object_get_blueprint_id(&component_address.into());

    let metadata_key = build_blueprint_metadata_key(namespace, &blueprint_id);
    let metadata_package_address = package_address_from_metadata(&metadata_key);

    assert_eq!(
        Some(blueprint_id.package_address),
        metadata_package_address,
        "The package address of {} is not approved under the key {} in the pool package metadata.",
        Runtime::bech32_encode_address(component_address),
        metadata_key
    );
}

pub fn package_address_from_metadata(key: &str) -> Option<PackageAddress> {
    let own_package: Package = Runtime::package_address().into();
    let metadata_value: GlobalAddress = own_package.get_metadata(key).ok()??;
    metadata_value.try_into().ok()
}

pub fn build_blueprint_metadata_key(namespace: &str, blueprint_id: &BlueprintId) -> String {
    let full_address = Runtime::bech32_encode_address(blueprint_id.package_address);
    let short_address = &full_address[full_address.len() - 6..];

    format!(
        "packages.{}.{}.{}",
        namespace,
        blueprint_id.blueprint_name.to_lowercase(),
        short_address
    )
}
