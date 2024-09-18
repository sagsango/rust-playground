// How to use cfg

fn feature_test() {
    /*
    How to add features in Cargo.toml:
    - In Cargo.toml, add the following:
        [features]
        feature1 = []
        feature2 = []
        feature3 = []
        featurename = ["dependency1", "dependency2"]

    2 wyas to enable a feature:
    1. cargo run --no-default-features --features feature1 --features feature2
    2. To enable a default feature, add the following in Cargo.toml:
        [features]
        default = ["feature1"]


    How to diasble a default feature in Cargo.toml:
    - default features can be disabled by adding the following in Cargo.toml:
        [package]
        default-features = false

     */
    #[cfg(feature = "feature1")]
    {
        println!("feature1 is enabled");
    }

    #[cfg(feature = "feature2")]
    {
        println!("feature2 is enabled");
    }

    #[cfg(feature = "feature3")]
    {
        println!("feature3 is enabled");
    }
}

pub fn test() {
    feature_test();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        feature_test();
    }
}