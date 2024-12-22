#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Fishpond struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Fishpond {
    id: u64,
    name: String,
    location: String,
    owner: String,
    contact: String,
    total_capacity: f64, // Total capacity in kilograms
    current_stock: f64,  // Current fish stock in kilograms
    created_at: u64,
}

// FishBatch struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FishBatch {
    id: u64,
    fishpond_id: u64,
    species: String,
    weight: f64,   // Weight in kilograms
    quantity: u64, // Number of fish in the batch
    arrival_date: u64,
}

// FeedingSchedule struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FeedingSchedule {
    id: u64,
    fishpond_id: u64,
    feed_type: String,
    quantity: f64,      // Quantity of feed in kilograms
    schedule_time: u64, // Timestamp of the feeding schedule
}

// Expense struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    fishpond_id: u64,
    category: String,
    amount: f64,
    description: String,
    date: u64,
}

// Harvest struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Harvest {
    id: u64,
    fishpond_id: u64,
    species: String,
    weight: f64, // Total weight harvested in kilograms
    revenue_per_kg: f64,
    total_revenue: f64,
    harvest_date: u64,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateFishpondPayload {
    name: String,
    location: String,
    owner: String,
    contact: String,
    total_capacity: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct AddFishBatchPayload {
    fishpond_id: u64,
    species: String,
    weight: f64,
    quantity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordFeedingPayload {
    fishpond_id: u64,
    feed_type: String,
    quantity: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordExpensePayload {
    fishpond_id: u64,
    category: String,
    amount: f64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordHarvestPayload {
    fishpond_id: u64,
    species: String,
    weight: f64,
    revenue_per_kg: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for Fishpond
impl Storable for Fishpond {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Fishpond {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for FishBatch
impl Storable for FishBatch {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FishBatch {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for FeedingSchedule
impl Storable for FeedingSchedule {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FeedingSchedule {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Expense
impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Harvest
impl Storable for Harvest {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Harvest {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static FISHPONDS: RefCell<StableBTreeMap<u64, Fishpond, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static FISH_BATCHES: RefCell<StableBTreeMap<u64, FishBatch, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static FEEDING_SCHEDULES: RefCell<StableBTreeMap<u64, FeedingSchedule, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static EXPENSES: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));

    static HARVESTS: RefCell<StableBTreeMap<u64, Harvest, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(14)))
        ));
}

// Functions

// Create Fishpond
#[ic_cdk::update]
fn create_fishpond(payload: CreateFishpondPayload) -> Result<Fishpond, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let fishpond_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let fishpond = Fishpond {
        id: fishpond_id,
        name: payload.name,
        location: payload.location,
        owner: payload.owner,
        contact: payload.contact,
        total_capacity: payload.total_capacity,
        current_stock: 0.0,
        created_at: time(),
    };

    FISHPONDS.with(|ponds| {
        ponds.borrow_mut().insert(fishpond_id, fishpond.clone());
    });

    Ok(fishpond)
}

// Add Fish Batch
#[ic_cdk::update]
fn add_fish_batch(payload: AddFishBatchPayload) -> Result<FishBatch, Message> {
    if payload.species.is_empty() || payload.weight <= 0.0 || payload.quantity == 0 {
        return Err(Message::InvalidPayload(
            "Invalid fish batch data".to_string(),
        ));
    }

    let pond_exists = FISHPONDS.with(|ponds| ponds.borrow().contains_key(&payload.fishpond_id));
    if !pond_exists {
        return Err(Message::NotFound("Fishpond not found".to_string()));
    }

    let batch_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let batch = FishBatch {
        id: batch_id,
        fishpond_id: payload.fishpond_id,
        species: payload.species,
        weight: payload.weight,
        quantity: payload.quantity,
        arrival_date: time(),
    };

    FISH_BATCHES.with(|batches| {
        batches.borrow_mut().insert(batch_id, batch.clone());
    });

    FISHPONDS.with(|ponds| {
        let mut ponds = ponds.borrow_mut();
        if let Some(fishpond) = ponds.get(&payload.fishpond_id) {
            let mut updated_pond = fishpond.clone();
            updated_pond.current_stock -= payload.weight;
            ponds.insert(payload.fishpond_id, updated_pond);
        }
    });

    Ok(batch)
}

// Record Feeding Schedule
#[ic_cdk::update]
fn record_feeding(payload: RecordFeedingPayload) -> Result<FeedingSchedule, Message> {
    if payload.feed_type.is_empty() || payload.quantity <= 0.0 {
        return Err(Message::InvalidPayload("Invalid feeding data".to_string()));
    }

    let pond_exists = FISHPONDS.with(|ponds| ponds.borrow().contains_key(&payload.fishpond_id));
    if !pond_exists {
        return Err(Message::NotFound("Fishpond not found".to_string()));
    }

    let schedule_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let schedule = FeedingSchedule {
        id: schedule_id,
        fishpond_id: payload.fishpond_id,
        feed_type: payload.feed_type,
        quantity: payload.quantity,
        schedule_time: time(),
    };

    FEEDING_SCHEDULES.with(|schedules| {
        schedules.borrow_mut().insert(schedule_id, schedule.clone());
    });

    Ok(schedule)
}

// Record Expense
#[ic_cdk::update]
fn record_expense(payload: RecordExpensePayload) -> Result<Expense, Message> {
    if payload.amount <= 0.0 {
        return Err(Message::InvalidPayload(
            "Invalid expense amount".to_string(),
        ));
    }

    let pond_exists = FISHPONDS.with(|ponds| ponds.borrow().contains_key(&payload.fishpond_id));
    if !pond_exists {
        return Err(Message::NotFound("Fishpond not found".to_string()));
    }

    let expense_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let expense = Expense {
        id: expense_id,
        fishpond_id: payload.fishpond_id,
        category: payload.category,
        amount: payload.amount,
        description: payload.description,
        date: time(),
    };

    EXPENSES.with(|expenses| {
        expenses.borrow_mut().insert(expense_id, expense.clone());
    });

    Ok(expense)
}

// Record Harvest
#[ic_cdk::update]
fn record_harvest(payload: RecordHarvestPayload) -> Result<Harvest, Message> {
    if payload.weight <= 0.0 || payload.revenue_per_kg <= 0.0 {
        return Err(Message::InvalidPayload("Invalid harvest data".to_string()));
    }

    let pond_exists = FISHPONDS.with(|ponds| ponds.borrow().contains_key(&payload.fishpond_id));
    if !pond_exists {
        return Err(Message::NotFound("Fishpond not found".to_string()));
    }

    let harvest_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let total_revenue = payload.weight * payload.revenue_per_kg;

    let harvest = Harvest {
        id: harvest_id,
        fishpond_id: payload.fishpond_id,
        species: payload.species,
        weight: payload.weight,
        revenue_per_kg: payload.revenue_per_kg,
        total_revenue,
        harvest_date: time(),
    };

    HARVESTS.with(|harvests| {
        harvests.borrow_mut().insert(harvest_id, harvest.clone());
    });

    FISHPONDS.with(|ponds| {
        let mut ponds = ponds.borrow_mut();
        if let Some(fishpond) = ponds.get(&payload.fishpond_id) {
            let mut updated_pond = fishpond.clone();
            updated_pond.current_stock += payload.weight;
            ponds.insert(payload.fishpond_id, updated_pond);
        }
    });

    Ok(harvest)
}

// Calculate Total Revenue
#[ic_cdk::query]
fn calculate_total_revenue(fishpond_id: u64) -> Result<f64, Message> {
    let pond_exists = FISHPONDS.with(|ponds| ponds.borrow().contains_key(&fishpond_id));
    if !pond_exists {
        return Err(Message::NotFound("Fishpond not found".to_string()));
    }

    let total_revenue: f64 = HARVESTS.with(|harvests| {
        harvests
            .borrow()
            .iter()
            .filter(|(_, harvest)| harvest.fishpond_id == fishpond_id)
            .map(|(_, harvest)| harvest.total_revenue)
            .sum()
    });

    Ok(total_revenue)
}

// Calculate Total Expenses
#[ic_cdk::query]
fn calculate_total_expenses(fishpond_id: u64) -> Result<f64, Message> {
    let pond_exists = FISHPONDS.with(|ponds| ponds.borrow().contains_key(&fishpond_id));
    if !pond_exists {
        return Err(Message::NotFound("Fishpond not found".to_string()));
    }

    let total_expenses: f64 = EXPENSES.with(|expenses| {
        expenses
            .borrow()
            .iter()
            .filter(|(_, expense)| expense.fishpond_id == fishpond_id)
            .map(|(_, expense)| expense.amount)
            .sum()
    });

    Ok(total_expenses)
}

// Exporting the candid interface
ic_cdk::export_candid!();
