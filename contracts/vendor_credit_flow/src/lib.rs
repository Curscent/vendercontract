#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Map, Symbol};

#[contract]
pub struct VendorCredit;

#[contracttype]
#[derive(Clone)]
pub struct Loan {
    pub supplier: Address,
    pub amount: i128,
    pub repaid: i128,
    pub active: bool,
}

const LOANS: Symbol = Symbol::short("LOANS");

#[contractimpl]
impl VendorCredit {

    // Vendor creates a loan request
    pub fn request_loan(env: Env, vendor: Address, supplier: Address, amount: i128) {
        vendor.require_auth();

        let mut loans: Map<Address, Loan> =
            env.storage().instance().get(&LOANS).unwrap_or(Map::new(&env));

        let loan = Loan {
            supplier,
            amount,
            repaid: 0,
            active: true,
        };

        loans.set(vendor, loan);
        env.storage().instance().set(&LOANS, &loans);
    }

    // Lender funds supplier directly
    pub fn fund(env: Env, vendor: Address) {
        let mut loans: Map<Address, Loan> = env.storage().instance().get(&LOANS).unwrap();
        let mut loan = loans.get(vendor.clone()).unwrap();

        if !loan.active {
            panic!("Loan not active");
        }

        // simulate payment success
        loan.active = true;

        loans.set(vendor, loan);
        env.storage().instance().set(&LOANS, &loans);
    }

    // Vendor repays loan
    pub fn repay(env: Env, vendor: Address, amount: i128) {
        vendor.require_auth();

        let mut loans: Map<Address, Loan> = env.storage().instance().get(&LOANS).unwrap();
        let mut loan = loans.get(vendor.clone()).unwrap();

        loan.repaid += amount;

        if loan.repaid >= loan.amount {
            loan.active = false;
        }

        loans.set(vendor, loan);
        env.storage().instance().set(&LOANS, &loans);
    }

    // View loan details
    pub fn get_loan(env: Env, vendor: Address) -> Loan {
        let loans: Map<Address, Loan> = env.storage().instance().get(&LOANS).unwrap();
        loans.get(vendor).unwrap()
    }
}