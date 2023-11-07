use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct DimensionsAvatar {
    name: String,
    collection_type: String,
    key_image_url: Url,
    nft_id: Decimal,
}

#[derive(NonFungibleData, ScryptoSbor)]
pub struct TradeBadge {
    minted_on: Epoch,
}

#[blueprint]
mod infinitedapp {
    struct InfiniteNft {
        avatar_vault: NonFungibleVault,
        avatar_price: Decimal,
        avatar_resource_manager: ResourceManager,
        collected_xrd: Vault,
        nft_id_counter: u64,
        receiver: Vec<ComponentAddress>,
    }

    impl InfiniteNft {
        pub fn instantiate_infinite() -> Global<InfiniteNft> {
            let infinite_bucket: NonFungibleBucket = ResourceBuilder::new_integer_non_fungible(
                OwnerRole::None,
            )
            .metadata(metadata!(
                init {
                    "name" => "Dimensions Classic Collection".to_owned(), locked;
                    "key_image_url" => Url::of("https://radixwalletimages.s3.us-east-2.amazonaws.com/gifs/infinite_favicon.png"), locked;
                }
            ))
            .mint_initial_supply([
                (
                    IntegerNonFungibleLocalId::new(1u64),
                    DimensionsAvatar {
                        name: "Cat".to_string(),
                        key_image_url: Url::of(
                            "https://radixwalletimages.s3.us-east-2.amazonaws.com/gifs/8.gif",
                        ),
                        collection_type: "OCI Cat".to_string(),
                        nft_id: Decimal::from(1),
                    },
                ),
                (
                    IntegerNonFungibleLocalId::new(2u64),
                    DimensionsAvatar {
                        name: "Dog".to_string(),
                        key_image_url: Url::of(
                            "https://radixwalletimages.s3.us-east-2.amazonaws.com/gifs/9.gif",
                        ),
                        collection_type: "Doge".to_string(),
                        nft_id: Decimal::from(2),
                    },
                ),
                (
                    IntegerNonFungibleLocalId::new(3u64),
                    DimensionsAvatar {
                        name: "Gnome".to_string(),
                        key_image_url: Url::of(
                            "https://radixwalletimages.s3.us-east-2.amazonaws.com/gifs/10.gif",
                        ),
                        collection_type: "Gnomes".to_string(),
                        nft_id: Decimal::from(3),
                    },
                ),
                (
                    IntegerNonFungibleLocalId::new(4u64),
                    DimensionsAvatar {
                        name: "Fractal".to_string(),
                        key_image_url: Url::of(
                            "https://radixwalletimages.s3.us-east-2.amazonaws.com/gifs/4.gif",
                        ),
                        collection_type: "Radical Fractals".to_string(),
                        nft_id: Decimal::from(4),
                    },
                ),
                (
                    IntegerNonFungibleLocalId::new(5u64),
                    DimensionsAvatar {
                        name: "Penguin 1".to_string(),
                        key_image_url: Url::of(
                            "https://radixwalletimages.s3.us-east-2.amazonaws.com/gifs/5.gif",
                        ),
                        collection_type: "Pengus".to_string(),
                        nft_id: Decimal::from(5),
                    },
                ),
                (
                    IntegerNonFungibleLocalId::new(6u64),
                    DimensionsAvatar {
                        name: "Penguin 2".to_string(),
                        key_image_url: Url::of(
                            "https://radixwalletimages.s3.us-east-2.amazonaws.com/gifs/6.gif",
                        ),
                        collection_type: "Pengus".to_string(),
                        nft_id: Decimal::from(6),
                    },
                ),
                (
                    IntegerNonFungibleLocalId::new(7u64),
                    DimensionsAvatar {
                        name: "Robot".to_string(),
                        key_image_url: Url::of(
                            "https://radixwalletimages.s3.us-east-2.amazonaws.com/gifs/7.gif",
                        ),
                        collection_type: "Rad Robo".to_string(),
                        nft_id: Decimal::from(7),
                    },
                ),
            ])
            .into();

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(InfiniteNft::blueprint_id());

            let avatar_trade_badge_manager = ResourceBuilder::new_integer_non_fungible::<
                DimensionsAvatar,
            >(OwnerRole::None)
            .metadata(metadata!(
                init {
                    "name" => "Avatar Trader".to_owned(), locked;
                }
            ))
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(deny_all);
            ))
            .burn_roles(burn_roles!(
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(deny_all);
            ))
            .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                non_fungible_data_updater => rule!(require(global_caller(component_address)));
                non_fungible_data_updater_updater => rule!(deny_all);
            ))
            .create_with_no_initial_supply();

            Self {
                avatar_vault: NonFungibleVault::with_bucket(infinite_bucket),
                avatar_price: 1.into(),
                collected_xrd: Vault::new(XRD),
                nft_id_counter: 0,
                receiver: Vec::new(),
                avatar_resource_manager: avatar_trade_badge_manager,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }

        pub fn buy_avatar(
            &mut self,
            mut payment: Bucket,
            key: NonFungibleLocalId,
        ) -> (NonFungibleBucket, Bucket) {
            self.collected_xrd.put(payment.take(self.avatar_price));

            let infinite_bucket = self.avatar_vault.as_non_fungible().take_non_fungible(&key);

            (infinite_bucket, payment)
        }
    }
}
