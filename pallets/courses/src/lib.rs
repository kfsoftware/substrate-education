#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
    use sp_std::prelude::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        sp_runtime::traits::Hash,
        traits::{Randomness, Currency, tokens::ExistenceRequirement},
        transactional,
    };
    use frame_support::traits::tokens::AssetId;
    use sp_io::hashing::blake2_128;
    use scale_info::TypeInfo;

    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};
    use frame_system::RawOrigin;
    use sp_runtime::traits::Bounded;
    use sp_runtime::{
        traits::{CheckedSub, SaturatedConversion, StaticLookup, Zero},
        DispatchError, Perbill, Percent,
    };

    type AccountOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Struct for holding Course information.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Course<T: Config> {
        pub name: Vec<u8>,
        pub owner: AccountOf<T>,
        pub image_url: Vec<u8>,
        pub category: Vec<u8>,
        pub description: Vec<u8>,
    }

    // Struct for holding Lecture information.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Lecture<T: Config> {
        pub name: Vec<u8>,
        pub contents: Vec<u8>,
        pub owner: AccountOf<T>,
    }

    // Struct for holding LectureCompleted information.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct LectureCompleted<T: Config> {
        pub owner: AccountOf<T>,
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    // Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the Courses pallet.
        type Currency: Currency<Self::AccountId>;

        /// The maximum amount of Courses a single account can own.
        #[pallet::constant]
        type MaxCourseOwned: Get<u32>;

        /// The type of Randomness we want to specify for this pallet.
        type CourseRandomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// Handles arithemtic overflow when incrementing the Course counter.
        CourseCntOverflow,
        /// An account cannot own more Courses than `MaxCourseCount`.
        ExceedMaxCourseOwned,
        /// Buyer cannot be the owner.
        BuyerIsCourseOwner,
        /// Cannot transfer a course to its owner.
        TransferToSelf,
        /// Handles checking whether the Course exists.
        CourseNotExist,
        /// Handles checking that the Course is owned by the account transferring, buying or setting a price for it.
        NotCourseOwner,
        /// Ensures the Course is for sale.
        CourseNotForSale,
        /// Ensures that the buying price is greater than the asking price.
        CourseBidPriceTooLow,
        /// Ensures that an account has enough funds to purchase a Course.
        NotEnoughBalance,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new Course was successfully created. \[sender, course_id\]
        Created(T::AccountId, T::Hash),
        /// A new Course was successfully updated. \[sender, course_id\]
        Updated(T::AccountId, T::Hash),
        /// Course name was successfully set. \[sender, course_id, new_name\]
        NameSet(T::AccountId, T::Hash, Vec<u8>),
        /// A Course was successfully transferred. \[from, to, course_id\]
        Transferred(T::AccountId, T::AccountId, T::Hash),
        /// A Course was successfully bought. \[buyer, seller, course_id, bid_price\]
        Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
    }

    // Storage items.

    #[pallet::storage]
    #[pallet::getter(fn course_cnt)]
    /// Keeps track of the number of Courses in existence.
    pub(super) type CourseCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn mint_assets)]
    /// Keeps track of the assets that can be generated
    pub(super) type MintAssets<T: Config> = StorageValue<_, Vec<T::AssetId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_nonce)]
    pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn courses)]
    /// Stores a Course's unique traits, owner and price.
    pub(super) type Courses<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Course<T>>;

    #[pallet::storage]
    #[pallet::getter(fn lectures)]
    /// Stores a Lecture unique traits, owner and price.
    pub(super) type Lectures<T: Config> = StorageDoubleMap<_, Twox64Concat, T::Hash, Twox64Concat, T::Hash, Lecture<T>>;


    #[pallet::storage]
    #[pallet::getter(fn lectures_completed)]
    /// Stores a Lecture unique traits, owner and price.
    pub(super) type LecturesCompleted<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Twox64Concat, T::AccountId>, // Account
            NMapKey<Twox64Concat, T::Hash>, // Course
            NMapKey<Twox64Concat, T::Hash>, // Lecture
        ),
        LectureCompleted<T>,
        OptionQuery,
    >;


    #[pallet::storage]
    #[pallet::getter(fn courses_owned)]
    /// Keeps track of what accounts own what Course.
    pub(super) type CoursesOwned<T: Config> =
    StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxCourseOwned>, ValueQuery>;

    // ACTION #11: Our pallet's genesis configuration.
    // Our pallet's genesis configuration.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub courses: Vec<(T::AccountId, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>,
    }

    // Required to implement default for GenesisConfig.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> GenesisConfig<T> {
            GenesisConfig { courses: vec![] }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            // When building a course from genesis config, we require the dna and gender to be supplied.
            for (acct, name, category, image_url, description) in &self.courses {
                let _ = <Pallet<T>>::mint(acct, name.clone(), category.clone(), image_url.clone(), description.clone());
            }
        }
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new unique course.
        ///
        /// The actual course creation is done in the `mint()` function.
        #[pallet::weight(100)]
        pub fn create_course(
            origin: OriginFor<T>,
            name: Vec<u8>,
            category: Vec<u8>,
            image_url: Vec<u8>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let course_id = Self::mint(&sender, name, category, image_url, description)?;
            log::info!("A course is born with ID Changed1: {:?}.", course_id);
            Self::deposit_event(Event::Created(sender, course_id));
            Ok(())
        }

        /// Create a new unique course.
        ///
        /// The actual course creation is done in the `mint()` function.
        #[pallet::weight(100)]
        pub fn set_assets(
            origin: OriginFor<T>,
            asset_ids: Vec<T::AssetId>,
        ) -> DispatchResult {
            let sender = ensure_root(origin)?;
            <MintAssets<T>>::put(
                asset_ids.clone()
            );
            // <LecturesCompleted<T>>::insert((sender.clone(), course_id, lecture_id), lecture_completed);
            log::info!("Setting asset ids: {:?}.", asset_ids.clone());
            Ok(())
        }

        /// Set lecture completed for a course.
        #[pallet::weight(100)]
        pub fn complete_lecture(origin: OriginFor<T>, course_id: T::Hash, lecture_id: T::Hash) -> DispatchResult {
            let sender = ensure_signed(origin.clone())?;
            let lecture_completed = LectureCompleted::<T> {
                owner: sender.clone(),
            };
            let admin = <T::Lookup as StaticLookup>::unlookup(sender.clone());
            let account_id = sender.clone();
            let random_hash = Self::_random_hash(&account_id);
            let asset_ids = <MintAssets<T>>::get();
            let asset_id_tmp = asset_ids[0];
            let encoded = Encode::encode(&asset_id_tmp);
            let asset_id = T::AssetId::decode(&mut &encoded[..]).unwrap();

            let amount = T::Balance::from(1u8);
            match pallet_assets::Pallet::<T>::mint_into(origin.clone(), asset_id, admin, amount) {
                Ok(_) => {}
                Err(error) => {
                    log::info!("Error: {:?}.", error.clone());
                }
            }
            <LecturesCompleted<T>>::insert((sender.clone(), course_id, lecture_id), lecture_completed);
            Ok(())
        }

        /// Add a lecture to a course.
        #[pallet::weight(100)]
        pub fn create_lecture(origin: OriginFor<T>, course_id: T::Hash, name: Vec<u8>, contents: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // ACTION #1a: Checking Course owner
            ensure!(Self::is_course_owner(&course_id, &sender)?, <Error<T>>::NotCourseOwner);
            let lecture = Lecture::<T> {
                name,
                contents,
                owner: sender.clone(),
            };
            let lecture_id = T::Hashing::hash_of(&lecture);

            <Lectures<T>>::insert(course_id, lecture_id, lecture);
            Ok(())
        }

        /// Set the name for a Course.
        ///
        /// Updates Course name and updates storage.
        #[pallet::weight(100)]
        pub fn update_name(
            origin: OriginFor<T>,
            course_id: T::Hash,
            new_name: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // ACTION #1a: Checking Course owner
            ensure!(Self::is_course_owner(&course_id, &sender)?, <Error<T>>::NotCourseOwner);

            let mut course = Self::courses(&course_id).ok_or(<Error<T>>::CourseNotExist)?;

            // ACTION #2: Set the Course price and update new Course infomation to storage.
            course.name = new_name.clone();
            <Courses<T>>::insert(&course_id, course);

            // ACTION #3: Deposit a "NameSet" event.
            // Deposit a "NameSet" event.
            Self::deposit_event(Event::NameSet(sender, course_id, new_name));

            Ok(())
        }
    }

    //** Our helper functions.**//
    impl<T: Config> Pallet<T> {
        // Helper to mint a Course.
        pub fn mint(
            owner: &T::AccountId,
            name: Vec<u8>,
            category: Vec<u8>,
            image_url: Vec<u8>,
            description: Vec<u8>,
        ) -> Result<T::Hash, Error<T>> {
            let course = Course::<T> {
                name,
                owner: owner.clone(),
                category,
                image_url,
                description,
            };

            let course_id = T::Hashing::hash_of(&course);

            // Performs this operation first as it may fail
            let new_cnt = Self::course_cnt().checked_add(1)
                .ok_or(<Error<T>>::CourseCntOverflow)?;

            // Performs this operation first because as it may fail
            <CoursesOwned<T>>::try_mutate(&owner, |course_vec| {
                course_vec.try_push(course_id)
            }).map_err(|_| <Error<T>>::ExceedMaxCourseOwned)?;

            <Courses<T>>::insert(course_id, course);
            <CourseCnt<T>>::put(new_cnt);
            Ok(course_id)
        }
        fn _random_hash(sender: &T::AccountId) -> T::Hash {
            let nonce = <Nonce<T>>::get();
            let seed = T::CourseRandomness::random_seed();

            T::Hashing::hash_of(&(seed, &sender, nonce))
        }

        // ACTION #1b
        pub fn is_course_owner(course_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
            match Self::courses(course_id) {
                Some(course) => Ok(course.owner == *acct),
                None => Err(<Error<T>>::CourseNotExist)
            }
        }
    }
}